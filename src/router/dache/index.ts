import { Express } from "express";
import DacheController from "../../controller/dache";
import NotFoundError from "../../error/notFound";

export default class DacheRouter {
  private readonly dacheController: DacheController;

  constructor(dacheController: DacheController) {
    this.dacheController = dacheController;

    this.registerGet.bind(this);
    this.registerSet.bind(this);
    this.registerDelete.bind(this);
  }

  registerRoutes(expressApp: Express) {
    this.registerGet(expressApp);
    this.registerSet(expressApp);
    this.registerDelete(expressApp);
  }

  private registerGet(expressApp: Express) {
    expressApp.get("/dache/:key", (request, response) => {
      const key = request.params.key;
      try {
        const value = this.dacheController.getValue(key);
        response.status(200).json({ key, value });
      } catch (error) {
        if (error instanceof NotFoundError) {
          response.status(404).send();
        } else {
          throw error;
        }
      }
    });
  }

  private registerSet(expressApp: Express) {
    expressApp.post("/dache/:key", async (request, response) => {
      const key = request.params.key;
      const value = request.body.value;
      const expiry = request.body.expiry;
      if (typeof value !== "string") {
        response.status(400).json({
          message: "value is required.",
        });
      } else if (
        expiry !== undefined &&
        expiry !== null &&
        typeof expiry !== "number"
      ) {
        response.status(400).json({
          message: "expiry should be omitted, null or a unix timestamp.",
        });
      } else {
        await this.dacheController.setValue({
          key,
          value,
          expiry: typeof expiry === "number" ? new Date(expiry * 1000) : null,
        });
        response.status(204).send();
      }
    });
  }

  private registerDelete(expressApp: Express) {
    expressApp.delete("/dache/:key", async (request, response) => {
      const key = request.params.key;
      await this.dacheController.deleteValue(key);
      response.status(204).send();
    });
  }
}
