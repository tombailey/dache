import Lock from "../index";
import AsyncLockImpl, * as AsyncLockImplType from "async-lock";

export default class AsyncLock implements Lock {
  private readonly lock: AsyncLockImplType;

  constructor(lock: AsyncLockImplType = new AsyncLockImpl()) {
    this.lock = lock;
  }

  acquire(key: string, withLockCallback: () => Promise<void>): Promise<void> {
    return this.lock.acquire(key, withLockCallback);
  }
}
