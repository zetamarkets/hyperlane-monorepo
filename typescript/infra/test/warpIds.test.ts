import { expect } from 'chai';
import { existsSync } from 'fs';

import { DEFAULT_REGISTRY_URI, getRegistry } from '../config/registry.js';
import { WarpRouteIds } from '../config/environments/mainnet3/warp/warpIds.js';

describe('Warp IDs', () => {
  it('Has all warp IDs in the registry', async function () {
    this.timeout(20_000);

    // Fork CI often doesn't have auth to query the GitHub-hosted registry reliably.
    // Prefer a local checkout if available (e.g. via CI's `checkout-registry` action).
    if (!process.env.REGISTRY_URI && !existsSync(DEFAULT_REGISTRY_URI)) {
      this.skip();
    }

    const registry = getRegistry();
    for (const warpId of Object.values(WarpRouteIds)) {
      // That's a long sentence!
      expect(
        registry.getWarpRoute(warpId),
        `Warp ID ${warpId} not in registry, the .registryrc or your local registry may be out of date`,
      ).to.not.be.null.and.not.be.undefined;
    }
  });
});
