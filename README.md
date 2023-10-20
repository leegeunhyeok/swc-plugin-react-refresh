# swc-plugin-react-refresh

Swc plugin implementation of [react-refresh/babel](https://www.npmjs.com/package/react-refresh)

> [!IMPORTANT]
> A plugin for developing bundlers
> and this plugin is experimental.

- [x] Explore React components in module
  - [x] Function expressions
  - [x] Arrow function expressions
  - [x] Class declarations
  - [x] Import statements(default, named)
  - [x] Export statements(default, named, named with declare)
- [x] Get component name from AST
- [x] Parse hook calls from AST
- [ ] Parse HoC(High Order Component) expressions(`React.memo`, `React.forwardedRef`, and Custom HoC)
- [ ] Generate signature key based on the order of hook call expressions

## Setup

```bash
npm install swc-plugin-react-refresh
# or yarn
yarn add swc-plugin-react-refresh
```

Add plugin to your swc options.

```ts
import { transform } from '@swc/core';

await transform(code, {
  jsc: {
    experimental: {
      plugins: [
        // Add plugin here
        ['swc-plugin-react-refresh', { skipEnvCheck: true }],
      ],
    },
  },
});
```

Finally, inject runtime code at the top of bundled source.

<details><summary>Runtime Code</summary>

```js
const RefreshRuntime = require('react-refresh/runtime');

const hmrContext = {};
const createHmrContext = (id) => {
  const state = {
    timeout: null,
    accepted: false,
    disposed: false,
  };

  const hot = {
    accept: () => {
      if (state.disposed) {
        throw new Error('HMR module was disposed');
      }
  
      if (state.accepted) {
        throw new Error('HMR already accepted');
      }

      state.accepted = true;
      state.timeout = setTimeout(() => {
        state.timeout = null;
        RefreshRuntime.performReactRefresh();
      }, 50);
    },
    dispose: () => {
      state.disposed = true;
    },
  };

  if (hmrContext[id]) {
    hmrContext[id].dispose();
  }

  hmrContext[id] = hot;

  return hot;
};

const isReactRefreshBoundary = (type) => {
  return RefreshRuntime.isLikelyComponentType(type) && !type.prototype.isReactComponent;
}

// `global` is platform dependent.
RefreshRuntime.injectIntoGlobalHook(global);
global.$RefreshReg$ = () => {};
global.$RefreshSig$ = () => (type) => type;
global.$RefreshRuntime$ = {
  isReactRefreshBoundary,
  getRegisterFunction: () => {
    return (type, id) => {
      if (isReactRefreshBoundary(type)) return;
      return RefreshRuntime.register(type, id);
    };
  },
  getCreateSignatureFunction: () => {
    return () => (type, id, forceReset, getCustomHooks) => {
      if (isReactRefreshBoundary(type)) return;
      return RefreshRuntime.createSignatureFunctionForTransform(type, id, forceReset, getCustomHooks);
    };
  },
};
global.__hmr__ = (type, id) => ({
  accept: () => {
    if (isReactRefreshBoundary(type)) {
      createHmrContext(id).accept();
    }
  },
});
```

</details>

## Development

```bash
cargo build

# release build
yarn build # target: wasm32-wasi
# or
cargo build-wasi --release # target: wasm32-wasi
cargo build-wasm32 --release # target: wasm32-unknown-unknown

# run unit tests
cargo test
```

## License

[MIT](./LICENSE)