import express from "express";
import morgan from "morgan";
import HealthRouter from "../router/health";
import NotFoundRouter from "../router/notFound";
import DacheRouter from "../router/dache";
import DacheController from "../controller/dache";
import handleError from "../middleware/error";

export async function createServer(dacheController: DacheController) {
  const expressApp = express();
  expressApp.disable("x-powered-by");
  expressApp.use(morgan("combined"));
  expressApp.use(express.json());

  new HealthRouter().registerRoutes(expressApp);
  new DacheRouter(dacheController).registerRoutes(expressApp);
  new NotFoundRouter().registerRoutes(expressApp);

  expressApp.use(handleError);

  return expressApp;
}
