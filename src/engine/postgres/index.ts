import DurabilityEngine, { CacheEntryMap } from "../";

import pg from "pg";
import CacheEntry from "../../entity/cacheEntry";

export const POSTGRES_DURABILITY_ENGINE = "postgres";

class PostgresDurabilityEngine implements DurabilityEngine {
  private readonly pgPool: pg.Pool;

  constructor(pgPool: pg.Pool) {
    this.pgPool = pgPool;
  }

  async initialize() {
    const client = await this.pgPool.connect();
    try {
      await client.query(
        `
          CREATE TABLE IF NOT EXISTS dache(
            key text PRIMARY KEY, 
            value text,
            expiry timestamp default null
          );
        `
      );
    } finally {
      client.release();
    }
  }

  async getAll(): Promise<CacheEntryMap> {
    const client = await this.pgPool.connect();
    try {
      const { rows } = await client.query<Record<string, any>>(
        `SELECT * FROM dache`
      );

      return new Map(
        rows.map((row) => {
          return [row["key"], { value: row["value"], expiry: row["expiry"] }];
        })
      );
    } finally {
      client.release();
    }
  }

  async remove(key: string): Promise<void> {
    const client = await this.pgPool.connect();
    try {
      await client.query(
        `
          DELETE FROM dache WHERE key=$1;
        `,
        [key]
      );
    } finally {
      client.release();
    }
  }

  async set(entry: CacheEntry): Promise<void> {
    const client = await this.pgPool.connect();
    try {
      await client.query<Record<string, string>>(
        `
          INSERT INTO dache (key, value, expiry) VALUES ($1, $2, $3) 
          ON CONFLICT (key) DO UPDATE SET value=$2, expiry = $3;
        `,
        [entry.key, entry.value, entry.expiry]
      );
    } finally {
      client.release();
    }
  }
}

export default async function getPostgresDurabilityEngine(): Promise<DurabilityEngine> {
  const envVarToConfig: Record<string, string | undefined> = {
    host: process.env["POSTGRES_HOST"],
    port: process.env["POSTGRES_PORT"],
    database: process.env["POSTGRES_DATABASE"],
    user: process.env["POSTGRES_USER"],
    password: process.env["POSTGRES_PASSWORD"],
  };

  const config = Object.keys(envVarToConfig).reduce(
    (config: Record<string, string>, key) => {
      const value = envVarToConfig[key];
      if (value === undefined) {
        throw new Error(`${key} is required for PostgresDurabilityEngine.`);
      }
      config[key] = value;
      return config;
    },
    {}
  );

  const engine = new PostgresDurabilityEngine(
    new pg.Pool({
      ...config,
      port: parseInt(config.port),
    })
  );
  await engine.initialize();
  return engine;
}
