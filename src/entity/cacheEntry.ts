type CacheEntry = {
  key: string;
  value: string;
  expiry: Date | null;
};

export default CacheEntry;
