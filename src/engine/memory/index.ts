import DurabilityEngine from "../";
import CacheEntry from "../../entity/cacheEntry";

export const MEMORY_DURABILITY_ENGINE = "memory";

class InMemoryDurabilityEngine implements DurabilityEngine {
  private readonly cacheMap: Map<CacheEntry["key"], Omit<CacheEntry, "key">>;

  constructor(cacheMap: Map<CacheEntry["key"], Omit<CacheEntry, "key">>) {
    this.cacheMap = cacheMap;
  }

  getAll() {
    return this.cacheMap;
  }

  remove(key: CacheEntry["key"]) {
    this.cacheMap.delete(key);
  }

  set(entry: CacheEntry) {
    this.cacheMap.set(entry.key, {
      value: entry.value,
      expiry: entry.expiry
    });
  }
}

export default function getMemoryDurabilityEngine(): DurabilityEngine {
  return new InMemoryDurabilityEngine(new Map());
}
