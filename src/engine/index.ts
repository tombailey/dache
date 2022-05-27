import getPostgresDurabilityEngine, {
  POSTGRES_DURABILITY_ENGINE,
} from "../engine/postgres";
import CacheEntry from "../entity/cacheEntry";

export const DURABILITY_ENGINE = "DURABILITY_ENGINE";

export type CacheEntryMap = Map<CacheEntry["key"], Omit<CacheEntry, "key">>;

type DurabilityEngine = {
  getAll: () => CacheEntryMap | Promise<CacheEntryMap>;
  set: (entry: CacheEntry) => void | Promise<void>;
  remove: (key: CacheEntry["key"]) => void | Promise<void>;
};

export default DurabilityEngine;

export async function getDurabilityEngine(): Promise<DurabilityEngine> {
  const engine = process.env[DURABILITY_ENGINE]?.toLowerCase();
  switch (engine) {
    case POSTGRES_DURABILITY_ENGINE:
      return await getPostgresDurabilityEngine();
    default:
      throw new Error(`${engine} is not recognized as a DurabilityEngine.`);
  }
}
