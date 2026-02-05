export default {
  skipFiles: ['test', 'mock', 'upgrade', 'interfaces'],
  istanbulReporter: ['lcov'],
  mocha: {
    enableTimeouts: false,
  },
};
