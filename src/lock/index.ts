type Lock = {
  acquire: (
    key: string,
    withLockCallback: () => Promise<void>
  ) => Promise<void>;
};

export default Lock;
