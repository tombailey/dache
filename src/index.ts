import { createServer } from "./server";
import { getDurabilityEngine } from "./engine";
import DacheController from "./controller/dache";

async function init() {
  const durabilityEngine = await getDurabilityEngine();
  const server = await createServer(
    new DacheController(await durabilityEngine.getAll(), durabilityEngine)
  );
  const port = process.env["PORT"] ?? 8080;
  server.listen(port);
}

init();
