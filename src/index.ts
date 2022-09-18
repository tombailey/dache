import { createServer } from "./server";
import { getDurabilityEngine } from "./engine";
import DacheController from "./controller/dache";
import Lock from "./lock";
import AsyncLock from "./lock/asyncLock";

async function init() {
  const durabilityEngine = await getDurabilityEngine();
  const lock: Lock = new AsyncLock();

  const server = await createServer(
    new DacheController(
      await durabilityEngine.getAll(),
      durabilityEngine,
      lock
    ),
    {
      redactKeys:
        (process.env["LOGGING_REDACT_KEYS"] ?? "false").toLowerCase() === "true",
    }
  );
  const port = process.env["PORT"] ?? 8080;
  server.listen(port);
}

init();
