import express from "express";
import morgan from "morgan";
import HealthRouter from "../router/health";
import NotFoundRouter from "../router/notFound";
import DacheRouter from "../router/dache";
import DacheController from "../controller/dache";
import handleError from "../middleware/error";
import LoggingConfig from "../config/logging";

export async function createServer(
  dacheController: DacheController,
  loggingConfig: LoggingConfig
) {
  const expressApp = express();
  expressApp.disable("x-powered-by");
  expressApp.use(express.json());

  morgan.token("redacted-path", (request) => {
    return new URL(request.url ?? "", "http://localhost")
    .pathname
    .replace(/\/dache\/.+/i, "/dache/*REDACTED*");
  });
  expressApp.use(
    morgan(
      loggingConfig.redactKeys
        ? ":remote-addr - :remote-user [:date[clf]] \":method :redacted-path HTTP/:http-version\" :status :res[content-length] \":referrer\" \":user-agent\""
        : "combined"
    )
  );

  new HealthRouter().registerRoutes(expressApp);
  new DacheRouter(dacheController).registerRoutes(expressApp);
  new NotFoundRouter().registerRoutes(expressApp);

  expressApp.use(handleError);

  return expressApp;
}
