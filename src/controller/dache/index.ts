import DurabilityEngine, { CacheEntryMap } from "../../engine";
import NotFoundError from "../../error/notFound";
import CacheEntry from "../../entity/cacheEntry";

export default class DacheController {
  private readonly cacheEntryMap: CacheEntryMap;
  private readonly durabilityEngine: DurabilityEngine;

  constructor(
    cacheEntryMap: CacheEntryMap,
    durabilityEngine: DurabilityEngine
  ) {
    this.cacheEntryMap = cacheEntryMap;
    this.durabilityEngine = durabilityEngine;
  }

  getValue(key: string): string {
    const entry = this.cacheEntryMap.get(key);
    if (entry === undefined) {
      throw new NotFoundError(`${key} was not found.`);
    } else if (entry.expiry !== null && entry.expiry.getTime() <= Date.now()) {
      // try to delete the expired entry but don't wait around for promise to be fulfilled
      // and ignore failure
      this.deleteValue(key).catch(console.error);
      throw new NotFoundError(`${key} was not found.`);
    } else {
      return entry.value;
    }
  }

  async setValue(entry: CacheEntry): Promise<void> {
    await this.durabilityEngine.set(entry);
    this.cacheEntryMap.set(entry.key, {
      value: entry.value,
      expiry: entry.expiry,
    });
  }

  async deleteValue(key: string): Promise<void> {
    await this.durabilityEngine.remove(key);
    this.cacheEntryMap.delete(key);
  }
}
