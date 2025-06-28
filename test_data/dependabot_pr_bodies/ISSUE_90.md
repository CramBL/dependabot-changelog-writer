Bumps the github-actions group with 5 updates in the / directory:

| Package                                                                               | From      | To       |
| ------------------------------------------------------------------------------------- | --------- | -------- |
| [github/codeql-action](https://github.com/github/codeql-action)                       | `3.28.19` | `3.29.0` |
| [docker/setup-buildx-action](https://github.com/docker/setup-buildx-action)           | `3.10.0`  | `3.11.1` |
| [actions/attest-build-provenance](https://github.com/actions/attest-build-provenance) | `2.3.0`   | `2.4.0`  |
| [actions/attest-sbom](https://github.com/actions/attest-sbom)                         | `2.2.0`   | `2.4.0`  |
| [sigstore/cosign-installer](https://github.com/sigstore/cosign-installer)             | `3.8.2`   | `3.9.0`  |


Updates `github/codeql-action` from 3.28.19 to 3.29.0
<details>
<summary>Release notes</summary>
<p><em>Sourced from <a href="https://github.com/github/codeql-action/releases">github/codeql-action's releases</a>.</em></p>
<blockquote>
<h2>v3.29.0</h2>
<h1>CodeQL Action Changelog</h1>
<p>See the <a href="https://github.com/github/codeql-action/releases">releases page</a> for the relevant changes to the CodeQL CLI and language packs.</p>
<h2>3.29.0 - 11 Jun 2025</h2>
<ul>
<li>Update default CodeQL bundle version to 2.22.0. <a href="https://redirect.github.com/github/codeql-action/pull/2925">#2925</a></li>
<li>Bump minimum CodeQL bundle version to 2.16.6. <a href="https://redirect.github.com/github/codeql-action/pull/2912">#2912</a></li>
</ul>
<p>See the full <a href="https://github.com/github/codeql-action/blob/v3.29.0/CHANGELOG.md">CHANGELOG.md</a> for more information.</p>
</blockquote>
</details>
<details>
<summary>Changelog</summary>
<p><em>Sourced from <a href="https://github.com/github/codeql-action/blob/main/CHANGELOG.md">github/codeql-action's changelog</a>.</em></p>
<blockquote>
<h1>CodeQL Action Changelog</h1>
<p>See the <a href="https://github.com/github/codeql-action/releases">releases page</a> for the relevant changes to the CodeQL CLI and language packs.</p>
<h2>[UNRELEASED]</h2>
<p>No user facing changes.</p>
<h2>3.29.0 - 11 Jun 2025</h2>
<ul>
<li>Update default CodeQL bundle version to 2.22.0. <a href="https://redirect.github.com/github/codeql-action/pull/2925">#2925</a></li>
<li>Bump minimum CodeQL bundle version to 2.16.6. <a href="https://redirect.github.com/github/codeql-action/pull/2912">#2912</a></li>
</ul>
<h2>3.28.19 - 03 Jun 2025</h2>
<ul>
<li>The CodeQL Action no longer includes its own copy of the extractor for the <code>actions</code> language, which is currently in public preview.
The <code>actions</code> extractor has been included in the CodeQL CLI since v2.20.6. If your workflow has enabled the <code>actions</code> language <em>and</em> you have pinned
your <code>tools:</code> property to a specific version of the CodeQL CLI earlier than v2.20.6, you will need to update to at least CodeQL v2.20.6 or disable
<code>actions</code> analysis.</li>
<li>Update default CodeQL bundle version to 2.21.4. <a href="https://redirect.github.com/github/codeql-action/pull/2910">#2910</a></li>
</ul>
<h2>3.28.18 - 16 May 2025</h2>
<ul>
<li>Update default CodeQL bundle version to 2.21.3. <a href="https://redirect.github.com/github/codeql-action/pull/2893">#2893</a></li>
<li>Skip validating SARIF produced by CodeQL for improved performance. <a href="https://redirect.github.com/github/codeql-action/pull/2894">#2894</a></li>
<li>The number of threads and amount of RAM used by CodeQL can now be set via the <code>CODEQL_THREADS</code> and <code>CODEQL_RAM</code> runner environment variables. If set, these environment variables override the <code>threads</code> and <code>ram</code> inputs respectively. <a href="https://redirect.github.com/github/codeql-action/pull/2891">#2891</a></li>
</ul>
<h2>3.28.17 - 02 May 2025</h2>
<ul>
<li>Update default CodeQL bundle version to 2.21.2. <a href="https://redirect.github.com/github/codeql-action/pull/2872">#2872</a></li>
</ul>
<h2>3.28.16 - 23 Apr 2025</h2>
<ul>
<li>Update default CodeQL bundle version to 2.21.1. <a href="https://redirect.github.com/github/codeql-action/pull/2863">#2863</a></li>
</ul>
<h2>3.28.15 - 07 Apr 2025</h2>
<ul>
<li>Fix bug where the action would fail if it tried to produce a debug artifact with more than 65535 files. <a href="https://redirect.github.com/github/codeql-action/pull/2842">#2842</a></li>
</ul>
<h2>3.28.14 - 07 Apr 2025</h2>
<ul>
<li>Update default CodeQL bundle version to 2.21.0. <a href="https://redirect.github.com/github/codeql-action/pull/2838">#2838</a></li>
</ul>
<h2>3.28.13 - 24 Mar 2025</h2>
<p>No user facing changes.</p>
<h2>3.28.12 - 19 Mar 2025</h2>
<ul>
<li>Dependency caching should now cache more dependencies for Java <code>build-mode: none</code> extractions. This should speed up workflows and avoid inconsistent alerts in some cases.</li>
</ul>
<!-- raw HTML omitted -->
</blockquote>
<p>... (truncated)</p>
</details>
<details>
<summary>Commits</summary>
<ul>
<li><a href="https://github.com/github/codeql-action/commit/ce28f5bb42b7a9f2c824e633a3f6ee835bab6858"><code>ce28f5b</code></a> Merge pull request <a href="https://redirect.github.com/github/codeql-action/issues/2926">#2926</a> from github/update-v3.29.0-e8799281c</li>
<li><a href="https://github.com/github/codeql-action/commit/bc251b7932638a7881a8db15d1aaf0151642af99"><code>bc251b7</code></a> Update changelog for v3.29.0</li>
<li><a href="https://github.com/github/codeql-action/commit/e8799281c8dee3b2e1aaed2c059e530fcfdc2d6d"><code>e879928</code></a> Merge pull request <a href="https://redirect.github.com/github/codeql-action/issues/2925">#2925</a> from github/update-bundle/codeql-bundle-v2.22.0</li>
<li><a href="https://github.com/github/codeql-action/commit/efd43b3097c094d883d91934155f0a32af09dff7"><code>efd43b3</code></a> Merge branch 'main' into update-bundle/codeql-bundle-v2.22.0</li>
<li><a href="https://github.com/github/codeql-action/commit/7cb9b16051842e6c23c8b9fbcf92481f92d0644a"><code>7cb9b16</code></a> Merge pull request <a href="https://redirect.github.com/github/codeql-action/issues/2912">#2912</a> from github/henrymercer/bump-minimum-codeql-2.16.6</li>
<li><a href="https://github.com/github/codeql-action/commit/3855117ba18b27e082b12e3e92e00d1b52aaa605"><code>3855117</code></a> Add changelog note</li>
<li><a href="https://github.com/github/codeql-action/commit/f5d4e2a7ca2a5826357748bb8743390a4775946f"><code>f5d4e2a</code></a> Update default bundle to codeql-bundle-v2.22.0</li>
<li><a href="https://github.com/github/codeql-action/commit/22deae890c55a1dc3ffba1aa20ad4148284e72d1"><code>22deae8</code></a> Update package-lock.json</li>
<li><a href="https://github.com/github/codeql-action/commit/df2a830ca4348a013f4804b56f41795f408f1e4e"><code>df2a830</code></a> Merge branch 'main' into henrymercer/bump-minimum-codeql-2.16.6</li>
<li><a href="https://github.com/github/codeql-action/commit/b1e4dc3db58c9601794e22a9f6d28d45461b9dbf"><code>b1e4dc3</code></a> Merge pull request <a href="https://redirect.github.com/github/codeql-action/issues/2916">#2916</a> from github/dependabot/npm_and_yarn/npm-5cdccdc43f</li>
<li>Additional commits viewable in <a href="https://github.com/github/codeql-action/compare/fca7ace96b7d713c7035871441bd52efbe39e27e...ce28f5bb42b7a9f2c824e633a3f6ee835bab6858">compare view</a></li>
</ul>
</details>
<br />

Updates `docker/setup-buildx-action` from 3.10.0 to 3.11.1
<details>
<summary>Release notes</summary>
<p><em>Sourced from <a href="https://github.com/docker/setup-buildx-action/releases">docker/setup-buildx-action's releases</a>.</em></p>
<blockquote>
<h2>v3.11.1</h2>
<ul>
<li>Fix <code>keep-state</code> not being respected by <a href="https://github.com/crazy-max"><code>@​crazy-max</code></a> in <a href="https://redirect.github.com/docker/setup-buildx-action/pull/429">docker/setup-buildx-action#429</a></li>
</ul>
<p><strong>Full Changelog</strong>: <a href="https://github.com/docker/setup-buildx-action/compare/v3.11.0...v3.11.1">https://github.com/docker/setup-buildx-action/compare/v3.11.0...v3.11.1</a></p>
<h2>v3.11.0</h2>
<ul>
<li>Keep BuildKit state support by <a href="https://github.com/crazy-max"><code>@​crazy-max</code></a> in <a href="https://redirect.github.com/docker/setup-buildx-action/pull/427">docker/setup-buildx-action#427</a></li>
<li>Remove aliases created when installing by default by <a href="https://github.com/hashhar"><code>@​hashhar</code></a> in <a href="https://redirect.github.com/docker/setup-buildx-action/pull/139">docker/setup-buildx-action#139</a></li>
<li>Bump <code>@​docker/actions-toolkit</code> from 0.56.0 to 0.62.1 in <a href="https://redirect.github.com/docker/setup-buildx-action/pull/422">docker/setup-buildx-action#422</a> <a href="https://redirect.github.com/docker/setup-buildx-action/pull/425">docker/setup-buildx-action#425</a></li>
</ul>
<p><strong>Full Changelog</strong>: <a href="https://github.com/docker/setup-buildx-action/compare/v3.10.0...v3.11.0">https://github.com/docker/setup-buildx-action/compare/v3.10.0...v3.11.0</a></p>
</blockquote>
</details>
<details>
<summary>Commits</summary>
<ul>
<li><a href="https://github.com/docker/setup-buildx-action/commit/e468171a9de216ec08956ac3ada2f0791b6bd435"><code>e468171</code></a> Merge pull request <a href="https://redirect.github.com/docker/setup-buildx-action/issues/429">#429</a> from crazy-max/fix-keep-state</li>
<li><a href="https://github.com/docker/setup-buildx-action/commit/a3e7502fd02828f4a26a8294ad2621a6c2204952"><code>a3e7502</code></a> chore: update generated content</li>
<li><a href="https://github.com/docker/setup-buildx-action/commit/b145473295476dbef957d01d109fe7810b511c95"><code>b145473</code></a> fix keep-state not being respected</li>
<li><a href="https://github.com/docker/setup-buildx-action/commit/18ce135bb5112fa8ce4ed6c17ab05699d7f3a5e0"><code>18ce135</code></a> Merge pull request <a href="https://redirect.github.com/docker/setup-buildx-action/issues/425">#425</a> from docker/dependabot/npm_and_yarn/docker/actions-to...</li>
<li><a href="https://github.com/docker/setup-buildx-action/commit/0e198e93af3b40a76583e851660b876e62b3a155"><code>0e198e9</code></a> chore: update generated content</li>
<li><a href="https://github.com/docker/setup-buildx-action/commit/05f3f3ac108784e8fb56815c12fbfcf2d0ed660f"><code>05f3f3a</code></a> build(deps): bump <code>@​docker/actions-toolkit</code> from 0.61.0 to 0.62.1</li>
<li><a href="https://github.com/docker/setup-buildx-action/commit/622913496df23a5293cfb3418e5836ee4dd28f3a"><code>6229134</code></a> Merge pull request <a href="https://redirect.github.com/docker/setup-buildx-action/issues/427">#427</a> from crazy-max/keep-state</li>
<li><a href="https://github.com/docker/setup-buildx-action/commit/c6f6a0702519e6c47b71b117b24c0c1c130fdf32"><code>c6f6a07</code></a> chore: update generated content</li>
<li><a href="https://github.com/docker/setup-buildx-action/commit/6c5e29d8485c56f3f8d1cb2197b657959dd6e032"><code>6c5e29d</code></a> skip builder creation if one already exists with the same name</li>
<li><a href="https://github.com/docker/setup-buildx-action/commit/548b2977492e10f459d0f0df8bee7ce3c5937792"><code>548b297</code></a> ci: keep-state check</li>
<li>Additional commits viewable in <a href="https://github.com/docker/setup-buildx-action/compare/b5ca514318bd6ebac0fb2aedd5d36ec1b5c232a2...e468171a9de216ec08956ac3ada2f0791b6bd435">compare view</a></li>
</ul>
</details>
<br />

Updates `actions/attest-build-provenance` from 2.3.0 to 2.4.0
<details>
<summary>Release notes</summary>
<p><em>Sourced from <a href="https://github.com/actions/attest-build-provenance/releases">actions/attest-build-provenance's releases</a>.</em></p>
<blockquote>
<h2>v2.4.0</h2>
<h2>What's Changed</h2>
<ul>
<li>Bump undici from 5.28.5 to 5.29.0 by <a href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a href="https://redirect.github.com/actions/attest-build-provenance/pull/633">actions/attest-build-provenance#633</a></li>
<li>Bump actions/attest from 2.3.0 to <a href="https://github.com/actions/attest/releases/tag/v2.4.0">2.4.0</a> by <a href="https://github.com/bdehamer"><code>@​bdehamer</code></a> in <a href="https://redirect.github.com/actions/attest-build-provenance/pull/654">actions/attest-build-provenance#654</a>
<ul>
<li>Includes support for the new well-known summary file which will accumulate paths to all attestations generated in a given workflow run</li>
</ul>
</li>
</ul>
<p><strong>Full Changelog</strong>: <a href="https://github.com/actions/attest-build-provenance/compare/v2.3.0...v2.4.0">https://github.com/actions/attest-build-provenance/compare/v2.3.0...v2.4.0</a></p>
</blockquote>
</details>
<details>
<summary>Commits</summary>
<ul>
<li><a href="https://github.com/actions/attest-build-provenance/commit/e8998f949152b193b063cb0ec769d69d929409be"><code>e8998f9</code></a> bump actions/attest from 2.3.0 to 2.4.0 (<a href="https://redirect.github.com/actions/attest-build-provenance/issues/654">#654</a>)</li>
<li><a href="https://github.com/actions/attest-build-provenance/commit/11c67f22cd5a3968528de1f8de4bb4487ee5306e"><code>11c67f2</code></a> Bump the npm-development group across 1 directory with 6 updates (<a href="https://redirect.github.com/actions/attest-build-provenance/issues/649">#649</a>)</li>
<li><a href="https://github.com/actions/attest-build-provenance/commit/39cb715ce0ddd23df1f705e863f642bfb72dfb2b"><code>39cb715</code></a> Bump the npm-development group across 1 directory with 7 updates (<a href="https://redirect.github.com/actions/attest-build-provenance/issues/641">#641</a>)</li>
<li><a href="https://github.com/actions/attest-build-provenance/commit/7d91c4030d8fdc376f87f022d8ca01fe8bf07fcd"><code>7d91c40</code></a> Bump undici from 5.28.5 to 5.29.0 (<a href="https://redirect.github.com/actions/attest-build-provenance/issues/633">#633</a>)</li>
<li><a href="https://github.com/actions/attest-build-provenance/commit/d848170917c12653fb344e617a79614f36d13e00"><code>d848170</code></a> Bump super-linter/super-linter in the actions-minor group (<a href="https://redirect.github.com/actions/attest-build-provenance/issues/640">#640</a>)</li>
<li><a href="https://github.com/actions/attest-build-provenance/commit/0ca36ea29fc5b46379679e3d2a9ce33a62c57e04"><code>0ca36ea</code></a> Bump the npm-development group with 7 updates (<a href="https://redirect.github.com/actions/attest-build-provenance/issues/582">#582</a>)</li>
<li><a href="https://github.com/actions/attest-build-provenance/commit/d82e7cd0c70d3e7b2615badc4d8824ca0b098d86"><code>d82e7cd</code></a> offboard from eslint in superlinter (<a href="https://redirect.github.com/actions/attest-build-provenance/issues/618">#618</a>)</li>
<li>See full diff in <a href="https://github.com/actions/attest-build-provenance/compare/db473fddc028af60658334401dc6fa3ffd8669fd...e8998f949152b193b063cb0ec769d69d929409be">compare view</a></li>
</ul>
</details>
<br />

Updates `actions/attest-sbom` from 2.2.0 to 2.4.0
<details>
<summary>Release notes</summary>
<p><em>Sourced from <a href="https://github.com/actions/attest-sbom/releases">actions/attest-sbom's releases</a>.</em></p>
<blockquote>
<h2>v2.4.0</h2>
<h2>What's Changed</h2>
<ul>
<li>Bump actions/attest from 2.2.1 to 2.3.0 in the actions-minor group by <a href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a href="https://redirect.github.com/actions/attest-sbom/pull/169">actions/attest-sbom#169</a></li>
<li>Bump undici from 5.28.5 to 5.29.0 by <a href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a href="https://redirect.github.com/actions/attest-sbom/pull/172">actions/attest-sbom#172</a></li>
<li>Bump actions/attest from 2.3.0 to 2.4.0 by <a href="https://github.com/bdehamer"><code>@​bdehamer</code></a> in <a href="https://redirect.github.com/actions/attest-sbom/pull/178">actions/attest-sbom#178</a>
<ul>
<li>Includes support for the new well-known summary file which will accumulate paths to all attestations generated in a given workflow run</li>
</ul>
</li>
</ul>
<p><strong>Full Changelog</strong>: <a href="https://github.com/actions/attest-sbom/compare/v2.2.0...v2.4.0">https://github.com/actions/attest-sbom/compare/v2.2.0...v2.4.0</a></p>
</blockquote>
</details>
<details>
<summary>Commits</summary>
<ul>
<li><a href="https://github.com/actions/attest-sbom/commit/bd218ad0dbcb3e146bd073d1d9c6d78e08aa8a0b"><code>bd218ad</code></a> bump actions/attest from 2.3.0 to 2.4.0 (<a href="https://redirect.github.com/actions/attest-sbom/issues/178">#178</a>)</li>
<li><a href="https://github.com/actions/attest-sbom/commit/bb6e4f037bdd3aeb83ba179ccaae7d665f113c3d"><code>bb6e4f0</code></a> Bump the npm-development group across 1 directory with 5 updates (<a href="https://redirect.github.com/actions/attest-sbom/issues/176">#176</a>)</li>
<li><a href="https://github.com/actions/attest-sbom/commit/4d6ee9870ba17c2238929ccc1e9ec5b6ab79f887"><code>4d6ee98</code></a> Bump undici from 5.28.5 to 5.29.0 (<a href="https://redirect.github.com/actions/attest-sbom/issues/172">#172</a>)</li>
<li><a href="https://github.com/actions/attest-sbom/commit/22e8414d9e5f06798e89f057145338bf0616eb8c"><code>22e8414</code></a> Bump the npm-development group with 6 updates (<a href="https://redirect.github.com/actions/attest-sbom/issues/174">#174</a>)</li>
<li><a href="https://github.com/actions/attest-sbom/commit/9d8c9cae4aae3141ee237a257b26961f1a10a34c"><code>9d8c9ca</code></a> Bump super-linter/super-linter in the actions-minor group (<a href="https://redirect.github.com/actions/attest-sbom/issues/173">#173</a>)</li>
<li><a href="https://github.com/actions/attest-sbom/commit/8253d9c0260c1d2c660b1cb0a29bc988db0b7f18"><code>8253d9c</code></a> Bump the npm-development group with 2 updates (<a href="https://redirect.github.com/actions/attest-sbom/issues/171">#171</a>)</li>
<li><a href="https://github.com/actions/attest-sbom/commit/c0e357efe340d37fa67e4f943e886afd311f81a5"><code>c0e357e</code></a> Bump actions/attest from 2.2.1 to 2.3.0 in the actions-minor group (<a href="https://redirect.github.com/actions/attest-sbom/issues/169">#169</a>)</li>
<li><a href="https://github.com/actions/attest-sbom/commit/bced3cb77f5931cbe80d4b577c963d167bde8035"><code>bced3cb</code></a> Bump the npm-development group with 5 updates (<a href="https://redirect.github.com/actions/attest-sbom/issues/170">#170</a>)</li>
<li><a href="https://github.com/actions/attest-sbom/commit/32cff21fddf6105644b2f8fc85e5f9fcfb3dc74f"><code>32cff21</code></a> offboard from eslint in superlinter (<a href="https://redirect.github.com/actions/attest-sbom/issues/167">#167</a>)</li>
<li><a href="https://github.com/actions/attest-sbom/commit/33f07475db7f65ddc36ef658ba5bf9d3a0686b88"><code>33f0747</code></a> Bump the npm-development group with 6 updates (<a href="https://redirect.github.com/actions/attest-sbom/issues/163">#163</a>)</li>
<li>Additional commits viewable in <a href="https://github.com/actions/attest-sbom/compare/115c3be05ff3974bcbd596578934b3f9ce39bf68...bd218ad0dbcb3e146bd073d1d9c6d78e08aa8a0b">compare view</a></li>
</ul>
</details>
<br />

Updates `sigstore/cosign-installer` from 3.8.2 to 3.9.0
<details>
<summary>Release notes</summary>
<p><em>Sourced from <a href="https://github.com/sigstore/cosign-installer/releases">sigstore/cosign-installer's releases</a>.</em></p>
<blockquote>
<h2>v3.9.0</h2>
<h2>What's Changed</h2>
<ul>
<li>Bump actions/setup-go from 5.4.0 to 5.5.0 by <a href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a href="https://redirect.github.com/sigstore/cosign-installer/pull/189">sigstore/cosign-installer#189</a></li>
<li>bump cosign install to use release v2.5.0 as default by <a href="https://github.com/cpanato"><code>@​cpanato</code></a> in <a href="https://redirect.github.com/sigstore/cosign-installer/pull/191">sigstore/cosign-installer#191</a></li>
</ul>
<p><strong>Full Changelog</strong>: <a href="https://github.com/sigstore/cosign-installer/compare/v3...v3.9.0">https://github.com/sigstore/cosign-installer/compare/v3...v3.9.0</a></p>
</blockquote>
</details>
<details>
<summary>Commits</summary>
<ul>
<li><a href="https://github.com/sigstore/cosign-installer/commit/fb28c2b6339dcd94da6e4cbcbc5e888961f6f8c3"><code>fb28c2b</code></a> bump cosign install to use release v2.5.0 as default (<a href="https://redirect.github.com/sigstore/cosign-installer/issues/191">#191</a>)</li>
<li><a href="https://github.com/sigstore/cosign-installer/commit/e9a05e6d32d7ed22b5656cd874ef31af58d05bfa"><code>e9a05e6</code></a> Bump actions/setup-go from 5.4.0 to 5.5.0 (<a href="https://redirect.github.com/sigstore/cosign-installer/issues/189">#189</a>)</li>
<li>See full diff in <a href="https://github.com/sigstore/cosign-installer/compare/3454372f43399081ed03b604cb2d021dabca52bb...fb28c2b6339dcd94da6e4cbcbc5e888961f6f8c3">compare view</a></li>
</ul>
</details>
<br />
