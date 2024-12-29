Bumps the cargo_dependencies group with 7 updates in the / directory:

| Package | From | To |
| --- | --- | --- |
| [serde](https://github.com/serde-rs/serde) | `1.0.215` | `1.0.216` |
| [chrono](https://github.com/chronotope/chrono) | `0.4.38` | `0.4.39` |
| [semver](https://github.com/dtolnay/semver) | `1.0.23` | `1.0.24` |
| [env_logger](https://github.com/rust-cli/env_logger) | `0.11.5` | `0.11.6` |
| [zip](https://github.com/zip-rs/zip2) | `2.2.1` | `2.2.2` |
| [wasm-bindgen-futures](https://github.com/rustwasm/wasm-bindgen) | `0.4.47` | `0.4.49` |
| [thiserror](https://github.com/dtolnay/thiserror) | `2.0.4` | `2.0.9` |


Updates `serde` from 1.0.215 to 1.0.216
<details>
<summary>Release notes</summary>
<p><em>Sourced from <a href="https://github.com/serde-rs/serde/releases">serde's releases</a>.</em></p>
<blockquote>
<h2>v1.0.216</h2>
<ul>
<li>Mark all generated impls with #[automatically_derived] to exclude from code coverage (<a href="https://redirect.github.com/serde-rs/serde/issues/2866">#2866</a>, <a href="https://redirect.github.com/serde-rs/serde/issues/2868">#2868</a>, thanks <a href="https://github.com/tdittr"><code>@​tdittr</code></a>)</li>
</ul>
</blockquote>
</details>
<details>
<summary>Commits</summary>
<ul>
<li><a href="https://github.com/serde-rs/serde/commit/ad8dd4148b5fabf0d643d4de604a0616f2796506"><code>ad8dd41</code></a> Release 1.0.216</li>
<li><a href="https://github.com/serde-rs/serde/commit/f91d2ed9aef4d7e86171333ff745f40ee6e83692"><code>f91d2ed</code></a> Merge pull request <a href="https://redirect.github.com/serde-rs/serde/issues/2868">#2868</a> from dtolnay/automaticallyderived</li>
<li><a href="https://github.com/serde-rs/serde/commit/9497463718813e83b69db4343bb6e8db5f28441e"><code>9497463</code></a> Mark all generated trait impls as #[automatically_derived]</li>
<li><a href="https://github.com/serde-rs/serde/commit/46e9ecfcdd5216929ebcf29e76adc072412c5380"><code>46e9ecf</code></a> Merge pull request <a href="https://redirect.github.com/serde-rs/serde/issues/2866">#2866</a> from tdittr/mark-visitors-as-generated</li>
<li><a href="https://github.com/serde-rs/serde/commit/e9c399c822aad494ab1e935a95f1a591a99b44ad"><code>e9c399c</code></a> Mark generated <code>impl de::Visitor</code> blocks as <code>#[automatically_derived]</code></li>
<li><a href="https://github.com/serde-rs/serde/commit/b9dbfcb4ac3b7a663d9efc6eb1387c62302a6fb4"><code>b9dbfcb</code></a> Switch out fnv in favor of foldhash in test</li>
<li><a href="https://github.com/serde-rs/serde/commit/c270e27a4d37f008c199523f223843e8659b7fd9"><code>c270e27</code></a> Use BuildHasher instead of Hasher in collection macros</li>
<li><a href="https://github.com/serde-rs/serde/commit/0307f604ea5ca91de1f65d1db075d5cc5abb00ad"><code>0307f60</code></a> Resolve question_mark clippy lint in build script</li>
<li>See full diff in <a href="https://github.com/serde-rs/serde/compare/v1.0.215...v1.0.216">compare view</a></li>
</ul>
</details>
<br />

Updates `chrono` from 0.4.38 to 0.4.39
<details>
<summary>Release notes</summary>
<p><em>Sourced from <a href="https://github.com/chronotope/chrono/releases">chrono's releases</a>.</em></p>
<blockquote>
<h2>0.4.39</h2>
<h2>What's Changed</h2>
<ul>
<li><a href="https://redirect.github.com/chronotope/chrono/issues/1577">#1577</a>: Changed years_since documentation to match its implementation by <a href="https://github.com/Taxalo"><code>@​Taxalo</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1578">chronotope/chrono#1578</a></li>
<li>Remove obsolete weird feature guard by <a href="https://github.com/djc"><code>@​djc</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1582">chronotope/chrono#1582</a></li>
<li>Fix format::strftime docs link by <a href="https://github.com/frederikhors"><code>@​frederikhors</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1581">chronotope/chrono#1581</a></li>
<li>Fix micros (optional) limit in and_hms_micro_opt by <a href="https://github.com/qrilka"><code>@​qrilka</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1584">chronotope/chrono#1584</a></li>
<li>Update windows-bindgen requirement from 0.56 to 0.57 by <a href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1589">chronotope/chrono#1589</a></li>
<li>native/date: Improve DelayedFormat doc re Panics by <a href="https://github.com/behnam-oneschema"><code>@​behnam-oneschema</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1590">chronotope/chrono#1590</a></li>
<li>Fix typo in rustdoc of <code>from_timestamp_nanos()</code> by <a href="https://github.com/sgoll"><code>@​sgoll</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1591">chronotope/chrono#1591</a></li>
<li>Update windows-bindgen requirement from 0.57 to 0.58 by <a href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1594">chronotope/chrono#1594</a></li>
<li>docs: document century cutoff for %y by <a href="https://github.com/MarcoGorelli"><code>@​MarcoGorelli</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1598">chronotope/chrono#1598</a></li>
<li>Checked <code>NaiveWeek</code> methods by <a href="https://github.com/bragov4ik"><code>@​bragov4ik</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1600">chronotope/chrono#1600</a></li>
<li>Impl serde::Serialize and serde::Deserialize for TimeDelta by <a href="https://github.com/Awpteamoose"><code>@​Awpteamoose</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1599">chronotope/chrono#1599</a></li>
<li>Derive <code>PartialEq</code>,<code>Eq</code>,<code>Hash</code>,<code>Copy</code> and <code>Clone</code> on <code>NaiveWeek</code> by <a href="https://github.com/DSeeLP"><code>@​DSeeLP</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1618">chronotope/chrono#1618</a></li>
<li>Support ohos tzdata since ver.oh35 by <a href="https://github.com/MirageLyu"><code>@​MirageLyu</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1613">chronotope/chrono#1613</a></li>
<li>Use Formatter::pad (instead of write_str) for Weekdays by <a href="https://github.com/horazont"><code>@​horazont</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1621">chronotope/chrono#1621</a></li>
<li>Fix typos by <a href="https://github.com/szepeviktor"><code>@​szepeviktor</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1623">chronotope/chrono#1623</a></li>
<li>Fix comment. by <a href="https://github.com/khuey"><code>@​khuey</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1624">chronotope/chrono#1624</a></li>
<li>chore: add <code>#[inline]</code> to <code>num_days</code> by <a href="https://github.com/CommanderStorm"><code>@​CommanderStorm</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1627">chronotope/chrono#1627</a></li>
<li>fix typo by <a href="https://github.com/futreall"><code>@​futreall</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1633">chronotope/chrono#1633</a></li>
<li>Update mod.rs by <a href="https://github.com/donatik27"><code>@​donatik27</code></a> in <a href="https://redirect.github.com/chronotope/chrono/pull/1638">chronotope/chrono#1638</a></li>
</ul>
</blockquote>
</details>
<details>
<summary>Commits</summary>
<ul>
<li><a href="https://github.com/chronotope/chrono/commit/8b863490d88ba098038392c8aa930012ffd0c439"><code>8b86349</code></a> Bump version to 0.4.39</li>
<li><a href="https://github.com/chronotope/chrono/commit/33aaebfc35794553960ed86f77a47d1d16fab988"><code>33aaebf</code></a> Update mod.rs</li>
<li><a href="https://github.com/chronotope/chrono/commit/65c47f377d5e0a2505cbbff46c8eec5ff2a13da9"><code>65c47f3</code></a> Update CHANGELOG.md</li>
<li><a href="https://github.com/chronotope/chrono/commit/ca8232ff4e51cd918b3a9d84bdca8c466ded3cdc"><code>ca8232f</code></a> Update licenses for unicode-ident 1.0.14</li>
<li><a href="https://github.com/chronotope/chrono/commit/1456fa0977e054ae7bd893936a1cace2acc5d0e2"><code>1456fa0</code></a> Apply suggestions from clippy 1.83</li>
<li><a href="https://github.com/chronotope/chrono/commit/1c7567b34d9f45c1a694add4424efa5718d1f9df"><code>1c7567b</code></a> Bump codecov/codecov-action from 4 to 5</li>
<li><a href="https://github.com/chronotope/chrono/commit/f9ffd6fbde2a2dbc02d42afdcbec5da84ba4e1e0"><code>f9ffd6f</code></a> add <code>#[inline]</code> to <code>num_days</code></li>
<li><a href="https://github.com/chronotope/chrono/commit/7974c60649351553e57727ea02f88b66f8835f0e"><code>7974c60</code></a> Fix comment.</li>
<li><a href="https://github.com/chronotope/chrono/commit/77d50b1fc1e28d592346532620c07622da3aa5d7"><code>77d50b1</code></a> Fix typos</li>
<li><a href="https://github.com/chronotope/chrono/commit/771c0477bf4fa3febc74f3c8cd8e7a25d9463a5c"><code>771c047</code></a> Use Formatter::pad (instead of write_str) for Weekdays</li>
<li>Additional commits viewable in <a href="https://github.com/chronotope/chrono/compare/v0.4.38...v0.4.39">compare view</a></li>
</ul>
</details>
<br />

Updates `semver` from 1.0.23 to 1.0.24
<details>
<summary>Release notes</summary>
<p><em>Sourced from <a href="https://github.com/dtolnay/semver/releases">semver's releases</a>.</em></p>
<blockquote>
<h2>1.0.24</h2>
<ul>
<li>Optimize Ord impls for semver::Prerelease and semver::BuildMetadata (<a href="https://redirect.github.com/dtolnay/semver/issues/328">#328</a>, thanks <a href="https://github.com/Eh2406"><code>@​Eh2406</code></a>)</li>
</ul>
</blockquote>
</details>
<details>
<summary>Commits</summary>
<ul>
<li><a href="https://github.com/dtolnay/semver/commit/6f4069dd66f437361631d076b4226dc540a08d12"><code>6f4069d</code></a> Release 1.0.24</li>
<li><a href="https://github.com/dtolnay/semver/commit/d03aba3a5126baffaecf285648fad0297a1449d7"><code>d03aba3</code></a> Touch up PR 328</li>
<li><a href="https://github.com/dtolnay/semver/commit/238757dae165ecd6b3fdcb8ab555bf9036234bf2"><code>238757d</code></a> Merge pull request <a href="https://redirect.github.com/dtolnay/semver/issues/328">#328</a> from Eh2406/master</li>
<li><a href="https://github.com/dtolnay/semver/commit/75856ef55b34f0201392b196bf4ad12e04610790"><code>75856ef</code></a> faster Ord when Eq</li>
<li><a href="https://github.com/dtolnay/semver/commit/89504eb28ca03d3f5f62d42e0f31e841b7635984"><code>89504eb</code></a> Prevent upload-artifact step from causing CI failure</li>
<li><a href="https://github.com/dtolnay/semver/commit/d1b17a9a09ede77e534e5947bb2845f0913461b1"><code>d1b17a9</code></a> Upload CI Cargo.lock for reproducing failures</li>
<li><a href="https://github.com/dtolnay/semver/commit/4ea60ae121bf0c8c5ad7e7aa5a9f663f305b7400"><code>4ea60ae</code></a> Resolve doc_lazy_continuation clippy lint</li>
<li><a href="https://github.com/dtolnay/semver/commit/f96f9d8b6f959ff72336ef74020782c5f237d62d"><code>f96f9d8</code></a> Merge pull request <a href="https://redirect.github.com/dtolnay/semver/issues/319">#319</a> from dtolnay/docsrs</li>
<li><a href="https://github.com/dtolnay/semver/commit/fc5c98dba460b3ec2a6175e84e0dd4497d32ef30"><code>fc5c98d</code></a> Rely on docs.rs to define --cfg=docsrs by default</li>
<li>See full diff in <a href="https://github.com/dtolnay/semver/compare/1.0.23...1.0.24">compare view</a></li>
</ul>
</details>
<br />

Updates `env_logger` from 0.11.5 to 0.11.6
<details>
<summary>Release notes</summary>
<p><em>Sourced from <a href="https://github.com/rust-cli/env_logger/releases">env_logger's releases</a>.</em></p>
<blockquote>
<h2>v0.11.6</h2>
<h2>[0.11.6] - 2024-12-20</h2>
<h3>Features</h3>
<ul>
<li>Opt-in file and line rendering</li>
</ul>
</blockquote>
</details>
<details>
<summary>Changelog</summary>
<p><em>Sourced from <a href="https://github.com/rust-cli/env_logger/blob/main/CHANGELOG.md">env_logger's changelog</a>.</em></p>
<blockquote>
<h2>[0.11.6] - 2024-12-20</h2>
<h3>Features</h3>
<ul>
<li>Opt-in file and line rendering</li>
</ul>
</blockquote>
</details>
<details>
<summary>Commits</summary>
<ul>
<li><a href="https://github.com/rust-cli/env_logger/commit/dc1a01a79729d9a43f9dfaf32080c5e7bdf05090"><code>dc1a01a</code></a> chore: Release</li>
<li><a href="https://github.com/rust-cli/env_logger/commit/65f81b3b6bcac25ce3de08187579ba38d0ea34f5"><code>65f81b3</code></a> docs: Update changelog</li>
<li><a href="https://github.com/rust-cli/env_logger/commit/77425992f658d00d41aafc29b3bc7cb2e93e0f80"><code>7742599</code></a> Merge pull request <a href="https://redirect.github.com/rust-cli/env_logger/issues/345">#345</a> from EriKWDev/main</li>
<li><a href="https://github.com/rust-cli/env_logger/commit/59229bce5331f23b021633a1c991672c93e0ec83"><code>59229bc</code></a> fix: Test result of everything enabled has changed</li>
<li><a href="https://github.com/rust-cli/env_logger/commit/b0d4760955fcfe30a4e4314fe0f1ba9952650855"><code>b0d4760</code></a> spelling + field names</li>
<li><a href="https://github.com/rust-cli/env_logger/commit/1bad1f59d73240de26cd6e632bb4cce4541ba0e1"><code>1bad1f5</code></a> feature: ability to display source file path and line number with default for...</li>
<li><a href="https://github.com/rust-cli/env_logger/commit/cc97bf76e37f72993553187434e7fcbeb62c8478"><code>cc97bf7</code></a> chore(deps): Update Rust Stable to v1.83 (<a href="https://redirect.github.com/rust-cli/env_logger/issues/343">#343</a>)</li>
<li><a href="https://github.com/rust-cli/env_logger/commit/240cd21de5b8b506f34dc8fdfbcaf49a73fb91c9"><code>240cd21</code></a> style: Make clippy happy</li>
<li><a href="https://github.com/rust-cli/env_logger/commit/da7ff822598df812102c56e5d0329c79d7bfa60b"><code>da7ff82</code></a> chore: Update from _rust template</li>
<li><a href="https://github.com/rust-cli/env_logger/commit/ab1d8549459a8f38897aa065a300279ead1019e4"><code>ab1d854</code></a> chore(deps): Update Rust crate snapbox to v0.6.20 (<a href="https://redirect.github.com/rust-cli/env_logger/issues/342">#342</a>)</li>
<li>Additional commits viewable in <a href="https://github.com/rust-cli/env_logger/compare/v0.11.5...v0.11.6">compare view</a></li>
</ul>
</details>
<br />

Updates `zip` from 2.2.1 to 2.2.2
<details>
<summary>Release notes</summary>
<p><em>Sourced from <a href="https://github.com/zip-rs/zip2/releases">zip's releases</a>.</em></p>
<blockquote>
<h2>v2.2.2</h2>
<h3><!-- raw HTML omitted -->🐛 Bug Fixes</h3>
<ul>
<li>rewrite the EOCD/EOCD64 detection to fix extreme performance regression (<a href="https://redirect.github.com/zip-rs/zip2/issues/247">#247</a>)</li>
</ul>
</blockquote>
</details>
<details>
<summary>Changelog</summary>
<p><em>Sourced from <a href="https://github.com/zip-rs/zip2/blob/master/CHANGELOG.md">zip's changelog</a>.</em></p>
<blockquote>
<h2><a href="https://github.com/zip-rs/zip2/compare/v2.2.1...v2.2.2">2.2.2</a> - 2024-12-16</h2>
<h3><!-- raw HTML omitted -->🐛 Bug Fixes</h3>
<ul>
<li>rewrite the EOCD/EOCD64 detection to fix extreme performance regression (<a href="https://redirect.github.com/zip-rs/zip2/issues/247">#247</a>)</li>
</ul>
</blockquote>
</details>
<details>
<summary>Commits</summary>
<ul>
<li><a href="https://github.com/zip-rs/zip2/commit/e074e09b83df2406ebc904200c1a4dc94345d7db"><code>e074e09</code></a> chore: release v2.2.2 (<a href="https://redirect.github.com/zip-rs/zip2/issues/270">#270</a>)</li>
<li><a href="https://github.com/zip-rs/zip2/commit/33c71ccc80f0eee0922aabdc681d257a5f753d07"><code>33c71cc</code></a> fix: rewrite the EOCD/EOCD64 detection to fix extreme performance regression ...</li>
<li><a href="https://github.com/zip-rs/zip2/commit/810d18a9a1bad3fad55de989d5cd90d51a23d3cf"><code>810d18a</code></a> deps: Relax dependency versions (<a href="https://redirect.github.com/zip-rs/zip2/issues/243">#243</a>)</li>
<li>See full diff in <a href="https://github.com/zip-rs/zip2/compare/v2.2.1...v2.2.2">compare view</a></li>
</ul>
</details>
<br />

Updates `wasm-bindgen-futures` from 0.4.47 to 0.4.49
<details>
<summary>Commits</summary>
<ul>
<li>See full diff in <a href="https://github.com/rustwasm/wasm-bindgen/commits">compare view</a></li>
</ul>
</details>
<br />

Updates `web-sys` from 0.3.74 to 0.3.76
<details>
<summary>Commits</summary>
<ul>
<li>See full diff in <a href="https://github.com/rustwasm/wasm-bindgen/commits">compare view</a></li>
</ul>
</details>
<br />

Updates `thiserror` from 2.0.4 to 2.0.9
<details>
<summary>Release notes</summary>
<p><em>Sourced from <a href="https://github.com/dtolnay/thiserror/releases">thiserror's releases</a>.</em></p>
<blockquote>
<h2>2.0.9</h2>
<ul>
<li>Work around <code>missing_inline_in_public_items</code> clippy restriction being triggered in macro-generated code (<a href="https://redirect.github.com/dtolnay/thiserror/issues/404">#404</a>)</li>
</ul>
<h2>2.0.8</h2>
<ul>
<li>Improve support for macro-generated <code>derive(Error)</code> call sites (<a href="https://redirect.github.com/dtolnay/thiserror/issues/399">#399</a>)</li>
</ul>
<h2>2.0.7</h2>
<ul>
<li>Work around conflict with #[deny(clippy::allow_attributes)] (<a href="https://redirect.github.com/dtolnay/thiserror/issues/397">#397</a>, thanks <a href="https://github.com/zertosh"><code>@​zertosh</code></a>)</li>
</ul>
<h2>2.0.6</h2>
<ul>
<li>Suppress deprecation warning on generated From impls (<a href="https://redirect.github.com/dtolnay/thiserror/issues/396">#396</a>)</li>
</ul>
<h2>2.0.5</h2>
<ul>
<li>Prevent deprecation warning on generated impl for deprecated type (<a href="https://redirect.github.com/dtolnay/thiserror/issues/394">#394</a>)</li>
</ul>
</blockquote>
</details>
<details>
<summary>Commits</summary>
<ul>
<li><a href="https://github.com/dtolnay/thiserror/commit/c535cecb6f8d98cbdc72f526fc4c8a8ae826e2a3"><code>c535cec</code></a> Release 2.0.9</li>
<li><a href="https://github.com/dtolnay/thiserror/commit/0a0516db7382a18212574dd0d04dceabe7d77b2d"><code>0a0516d</code></a> Merge pull request <a href="https://redirect.github.com/dtolnay/thiserror/issues/404">#404</a> from dtolnay/fromfn</li>
<li><a href="https://github.com/dtolnay/thiserror/commit/e5169bb127f835d5fc390a5ca9acd673d075e21e"><code>e5169bb</code></a> Unspan From impl contents</li>
<li><a href="https://github.com/dtolnay/thiserror/commit/c0083752681756b7ad1aae2e6a15717d3d27118d"><code>c008375</code></a> FIx typo in ui test</li>
<li><a href="https://github.com/dtolnay/thiserror/commit/2bd29821f4ea339c60edfcf4734499d68128eb2e"><code>2bd2982</code></a> Release 2.0.8</li>
<li><a href="https://github.com/dtolnay/thiserror/commit/a7de3ab22d01922e050aad4202d71a4bfb577598"><code>a7de3ab</code></a> Merge pull request <a href="https://redirect.github.com/dtolnay/thiserror/issues/399">#399</a> from dtolnay/respan</li>
<li><a href="https://github.com/dtolnay/thiserror/commit/f1243a0ceb1c596f808c8df8d1240ed301135a1b"><code>f1243a0</code></a> Fix spans on macro-generated bindings and format variables</li>
<li><a href="https://github.com/dtolnay/thiserror/commit/6a07345135802344616a09584c94e2f4bbceb466"><code>6a07345</code></a> Add regression test for issue 398</li>
<li><a href="https://github.com/dtolnay/thiserror/commit/9c0f2d230da33dfec248d48d82c25a2ad19e6129"><code>9c0f2d2</code></a> Release 2.0.7</li>
<li><a href="https://github.com/dtolnay/thiserror/commit/2deec96fc0de605d114d3860f29d1d066ad4151e"><code>2deec96</code></a> Merge pull request 397 from zertosh/from_allow_expect</li>
<li>Additional commits viewable in <a href="https://github.com/dtolnay/thiserror/compare/2.0.4...2.0.9">compare view</a></li>
</ul>
</details>
<br />

