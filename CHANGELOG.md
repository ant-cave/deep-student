# Changelog | 更新日志

All notable changes to this project will be documented in this file.

本项目的所有重要变更都将记录在此文件中。

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，
本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [0.9.39](https://github.com/ant-cave/deep-student/compare/v0.9.38...v0.9.39) (2026-06-30)


### Features

* add FTP storage configuration and enhance cloud storage section ([acbab11](https://github.com/ant-cave/deep-student/commit/acbab11afd5b76a146a6a20251f1a54ed711f458))
* add notification plugin and enhance data governance features ([4401496](https://github.com/ant-cave/deep-student/commit/44014968c4fd2c8ea3c9b336de25adb85ccd8daa))
* **app:** 添加桌面端侧栏支持和优化模板管理页面 ([e72c111](https://github.com/ant-cave/deep-student/commit/e72c111213d0c136fc50ec8c91a05e53f836ea59))
* **app:** 添加桌面端侧栏支持和优化模板管理页面 ([07a80e7](https://github.com/ant-cave/deep-student/commit/07a80e78e7d7e6e451305120b362656b25b1803d))
* auto assign models on API key configuration ([#107](https://github.com/ant-cave/deep-student/issues/107)) ([009cc75](https://github.com/ant-cave/deep-student/commit/009cc75d48b8e3eaf5fe5d89e87e4827110ac961))
* **backend:** enhance chat pipeline helpers and prompt builder ([c67464b](https://github.com/ant-cave/deep-student/commit/c67464b7a7ff73e8009796360d912d660e98c548))
* **backend:** enhance chat pipeline helpers and prompt builder ([9ad3f2b](https://github.com/ant-cave/deep-student/commit/9ad3f2b485af2545e4dcf528ea5a49061d9593f1))
* **backend:** enhance LLM manager, builtin vendors & model capabilities ([eb1ae74](https://github.com/ant-cave/deep-student/commit/eb1ae7477c748af5ab53c3df92731574cc166e74))
* **backend:** enhance LLM manager, builtin vendors & model capabilities ([2dce3e7](https://github.com/ant-cave/deep-student/commit/2dce3e79d8b42a087ddfb7a88f3183bacb80fa7d))
* **backend:** enhance pipeline services & multimodal infrastructure ([3b38e13](https://github.com/ant-cave/deep-student/commit/3b38e13aa89b8d9b09d274c632558e608cc79143))
* **backend:** enhance pipeline services & multimodal infrastructure ([05d7879](https://github.com/ant-cave/deep-student/commit/05d7879950723a672c4a2ab031dd04842afc581d))
* **backend:** enhance sync infrastructure, data governance & migration ([b9c4b01](https://github.com/ant-cave/deep-student/commit/b9c4b01e62a0a08e66b7b063f4ddda46a5a15890))
* **backend:** enhance sync infrastructure, data governance & migration ([3679562](https://github.com/ant-cave/deep-student/commit/36795623f0afb676c6f715f383eab8622fb3607c))
* **backend:** migrate Gemini models to JSON registry ([6bdb90c](https://github.com/ant-cave/deep-student/commit/6bdb90c87350a40dc84d2320177a337208f31c41))
* **backend:** migrate Gemini models to JSON registry ([389844a](https://github.com/ant-cave/deep-student/commit/389844a81f2681b31bc5a11ae576c130d61bc653))
* **blocking-interaction:** unified blocking bar for approval/ask-user/tool-limit ([389547b](https://github.com/ant-cave/deep-student/commit/389547b3887c6bf36ff91f0119d4c925ba69ecb4))
* **blocking-interaction:** unified blocking bar for approval/ask-user/tool-limit ([c7ab582](https://github.com/ant-cave/deep-student/commit/c7ab5824cde3a7060bb9f93e99b3a9f8e61179ee))
* **chat_v2:** add title_locked field to prevent auto-summary from overwriting user-set titles ([91ecc64](https://github.com/ant-cave/deep-student/commit/91ecc64e883ca81afa6aa1d2ffca88679b1b9b4d))
* **chat_v2:** add title_locked field to prevent auto-summary from overwriting user-set titles ([4c755fb](https://github.com/ant-cave/deep-student/commit/4c755fbb35fc9d66b037f1e937d2245cc6bf0e12))
* **chat-queue:** add i18n + useQueueSettings hook + AppTab toggles ([fd11689](https://github.com/ant-cave/deep-student/commit/fd11689b3e4f5a7c00548f2ed620a34cedacd0d9))
* **chat-queue:** add i18n + useQueueSettings hook + AppTab toggles ([9f6fb33](https://github.com/ant-cave/deep-student/commit/9f6fb33f6b01120f32ce6c2436683cc80aa828ef))
* **chat-queue:** merge queue+steer into single setting toggle ([939f81e](https://github.com/ant-cave/deep-student/commit/939f81ed231be84e5ca4056dc66df555e887018c))
* **chat-queue:** merge queue+steer into single setting toggle ([7a83a34](https://github.com/ant-cave/deep-student/commit/7a83a34752993cfe3a99bcb7fbf2c078ca330253))
* **chat-queue:** propagate steered flag from queue to user message and render badge ([280c088](https://github.com/ant-cave/deep-student/commit/280c088a726c2f24496b920c1bdceaddca7b4324))
* **chat-queue:** propagate steered flag from queue to user message and render badge ([0ea3464](https://github.com/ant-cave/deep-student/commit/0ea34649ab86839367179605910ba6b9766e10ea))
* **chat-store:** add canEnqueue/canEnqueueSwap/canDequeue guards ([8985754](https://github.com/ant-cave/deep-student/commit/8985754096dd5c7e9aa81a660c643296d0c8a329))
* **chat-store:** add canEnqueue/canEnqueueSwap/canDequeue guards ([752d97f](https://github.com/ant-cave/deep-student/commit/752d97f8052c9b73453e9ae4867ca92943a1b7b4))
* **chat-store:** add queue mutation actions (enqueue/remove/clear/promote/retry/recall/swap) ([239f659](https://github.com/ant-cave/deep-student/commit/239f65902bc360961f8d2f40b06005e3f271eb42))
* **chat-store:** add queue mutation actions (enqueue/remove/clear/promote/retry/recall/swap) ([616933e](https://github.com/ant-cave/deep-student/commit/616933e7dc45889ee08c5732f54e908fb80dd704))
* **chat-store:** add QueuedMessage type and queue state fields ([36b30c3](https://github.com/ant-cave/deep-student/commit/36b30c36df4c0e6deeea87a5b262629604f64c50))
* **chat-store:** add QueuedMessage type and queue state fields ([e7a0d85](https://github.com/ant-cave/deep-student/commit/e7a0d8572ae715e972a66937752cd28a81d3c7cf))
* **chat-store:** wire queueActions + maybeDequeue + idle-transition subscription ([62c8505](https://github.com/ant-cave/deep-student/commit/62c85054529caeb200d9fb5037f5cfbfec5dbdaf))
* **chat-store:** wire queueActions + maybeDequeue + idle-transition subscription ([6847e32](https://github.com/ant-cave/deep-student/commit/6847e322ec86a99e64397b352d3ea8deebad3cd6))
* **chat-v2:** add LLM Output Playground dev tool ([e857d4a](https://github.com/ant-cave/deep-student/commit/e857d4a56058a63f8258d1867d6b8fcab3fcb788))
* **chat-v2:** add LLM Output Playground dev tool ([bca922c](https://github.com/ant-cave/deep-student/commit/bca922c92ff7ddbb1a657c6991c69dd894fecdb8))
* **chat/motion:** respect prefers-reduced-motion across thinking, todo, workspace surfaces ([2c80623](https://github.com/ant-cave/deep-student/commit/2c806230736424d12080cf03c5d9e0a2bc0ba6fb))
* **chat/motion:** respect prefers-reduced-motion across thinking, todo, workspace surfaces ([7f94b70](https://github.com/ant-cave/deep-student/commit/7f94b70ad6bb245a04f8bea3d9639511771d2887))
* **chat/timeline:** live thinking duration with 1Hz tick (5s under reduced-motion) ([f3f975f](https://github.com/ant-cave/deep-student/commit/f3f975fa25d110097ff8d97c211dc1f972600dbb))
* **chat/timeline:** live thinking duration with 1Hz tick (5s under reduced-motion) ([261720a](https://github.com/ant-cave/deep-student/commit/261720a51b93881a2447ad6ee7728e7b526dcab1))
* **chat/translation:** rewrite TranslationPopover with streaming NDJSON & LRU cache ([c72f97b](https://github.com/ant-cave/deep-student/commit/c72f97b376d32c71606bee42d154e9fd5fc180af))
* **chat/translation:** rewrite TranslationPopover with streaming NDJSON & LRU cache ([7608c01](https://github.com/ant-cave/deep-student/commit/7608c01c16e30b4217d755bfe94db22a5d73fb71))
* **chat:** add AgentTaskPanel and improve chat UI ([8f1d657](https://github.com/ant-cave/deep-student/commit/8f1d657fc76db52bb43fad5ffb98a1f18ecbd9d6))
* **chat:** add AgentTaskPanel and improve chat UI ([a4a9a4e](https://github.com/ant-cave/deep-student/commit/a4a9a4eafb723df4b2822e4887ff8d60a8c357ef))
* **chat:** add ExplainPopover and TranslationPopover to selection toolbar ([5c48932](https://github.com/ant-cave/deep-student/commit/5c48932ee9e9b6358a18a5eaf262c93a4e70b0aa))
* **chat:** add ExplainPopover and TranslationPopover to selection toolbar ([a50ed6a](https://github.com/ant-cave/deep-student/commit/a50ed6a6212d19ce32701aa25caafa7386cac7bd))
* **chat:** add FlowToken streaming renderer ([2068206](https://github.com/ant-cave/deep-student/commit/206820649ad1622cf89adf47cd1f7c71b1a41680))
* **chat:** add FlowToken streaming renderer ([039e5d1](https://github.com/ant-cave/deep-student/commit/039e5d1b477d5afa2f25e1edae49ac00d035b110))
* **chat:** add shell UI components, tool duration & activity timeline ([d1d5469](https://github.com/ant-cave/deep-student/commit/d1d54695a97580083b7f208c876a3f0ba68400ad))
* **chat:** add shell UI components, tool duration & activity timeline ([fb82fdc](https://github.com/ant-cave/deep-student/commit/fb82fdc9f8b8acecad67856c79a8090eccbe65c5))
* **chat:** anchor composer panels above input ([e942c03](https://github.com/ant-cave/deep-student/commit/e942c0335c9236d82a1c0cbdf837c038ae3fc950))
* **chat:** anchor composer panels above input ([a4e96b9](https://github.com/ant-cave/deep-student/commit/a4e96b9dc08147ef210659d0290efe087911f391))
* **chat:** enhance input bar, blocking bars, model picker & queue ([f230a97](https://github.com/ant-cave/deep-student/commit/f230a97a516e88e0308828c66ee83b585dff93e3))
* **chat:** enhance input bar, blocking bars, model picker & queue ([da8a3ca](https://github.com/ant-cave/deep-student/commit/da8a3caacee7ad28b42ed65e7fd65e2ec8a17f26))
* **chat:** enhance input bar, TokenUsageDisplay & add useSmoothWheel hook ([6d388d5](https://github.com/ant-cave/deep-student/commit/6d388d5f2b7efcc80910e442da618ef3463cde2e))
* **chat:** enhance input bar, TokenUsageDisplay & add useSmoothWheel hook ([481c404](https://github.com/ant-cave/deep-student/commit/481c40468b138045b6a0508ffcea51b8cce14223))
* **chat:** IconSwap component + new-message entrance + transitions polish ([1d35f21](https://github.com/ant-cave/deep-student/commit/1d35f21a1af2cbb110187e374fbc493d12031f55))
* **chat:** IconSwap component + new-message entrance + transitions polish ([bf878af](https://github.com/ant-cave/deep-student/commit/bf878af671c60a681a3acc580ad2cc9af68f411b))
* **chat:** improve blocking flows and preview feedback ([c9bb80e](https://github.com/ant-cave/deep-student/commit/c9bb80ee8fcfc6be64a862cc47475078531f6dee))
* **chat:** improve blocking flows and preview feedback ([f20322b](https://github.com/ant-cave/deep-student/commit/f20322b97bf76e335850fa847615f08f94fe119a))
* **chat:** improve renderers, markdown styles, and CodeBlockShell ([00a8441](https://github.com/ant-cave/deep-student/commit/00a84412554f73458ff3ea52f8cd93766566b0b2))
* **chat:** improve renderers, markdown styles, and CodeBlockShell ([e17d9b9](https://github.com/ant-cave/deep-student/commit/e17d9b9b2e9a707023d8ba257a250668522210a4))
* **chat:** pass streamSmoothingPreset/blockId/messageId through renderers ([af7e756](https://github.com/ant-cave/deep-student/commit/af7e7564d2bd7f419085cd95ee694d7a4960c3e8))
* **chat:** pass streamSmoothingPreset/blockId/messageId through renderers ([41c45ba](https://github.com/ant-cave/deep-student/commit/41c45bafdb160710845b74c1a87a68f905eeaddf))
* **chat:** refactor chat UI components - message items, input bar, plugins ([d1baaad](https://github.com/ant-cave/deep-student/commit/d1baaad2123d4c71fa7a6c7f7b0159ab6eba197f))
* **chat:** refactor chat UI components - message items, input bar, plugins ([c40be87](https://github.com/ant-cave/deep-student/commit/c40be87e6994b2896d089bcf21b041a45e857af3))
* **cloud-storage:** add FTP/FTPS storage support ([#92](https://github.com/ant-cave/deep-student/issues/92)) ([8169aaf](https://github.com/ant-cave/deep-student/commit/8169aafeaa774e31dfda2361a11425b0c630a8fb))
* **cloud-storage:** add insecure connection warning for WebDAV and S3 ([#108](https://github.com/ant-cave/deep-student/issues/108)) ([eef3f4a](https://github.com/ant-cave/deep-student/commit/eef3f4ae31b6a380a4f0ab036b93445b53dfb9f6))
* **core:** comprehensive platform enhancements with pomodoro mini-window, todo improvements, and sync optimizations ([e84a150](https://github.com/ant-cave/deep-student/commit/e84a1500798757ea5422227d86ae0705c3c6949d))
* **debug:** make debug toggle button draggable ([14d0417](https://github.com/ant-cave/deep-student/commit/14d0417b4ba8aec1560e16bcd221f6367ffac5bd))
* **debug:** make debug toggle button draggable ([54d1ba8](https://github.com/ant-cave/deep-student/commit/54d1ba8df273503f204439578dd7693cd87a12ec))
* **docs:** add comprehensive agent documentation for DeepStudent project ([bb04476](https://github.com/ant-cave/deep-student/commit/bb044766ad011dc1dac1a8206197bce0ab42e955))
* **docs:** update agent status documentation with recent findings and optimizations ([dab9ad9](https://github.com/ant-cave/deep-student/commit/dab9ad9d11536b5b556f81a88fab801347eb4a78))
* enhance conflict resolution and deduplication in sync process ([24126b3](https://github.com/ant-cave/deep-student/commit/24126b3be9dd182ee68e0119f2108535476ba945))
* FTP cloud storage, quarantine batch operations, and global data cleanup ([#106](https://github.com/ant-cave/deep-student/issues/106)) ([e41f3a7](https://github.com/ant-cave/deep-student/commit/e41f3a78e2f64ab44514475d1a0e611ba74e0fbb))
* **icons:** add Phosphor migration abstraction layer (adapter, mapping, barrel) ([3b98958](https://github.com/ant-cave/deep-student/commit/3b98958898ec11bcb514111c71463454c4e9f06a))
* **icons:** add Phosphor migration abstraction layer (adapter, mapping, barrel) ([9a0d705](https://github.com/ant-cave/deep-student/commit/9a0d7058d260d17b898ae83b3857bc233b417038))
* improve chat retry feedback and desktop shell behavior ([191884c](https://github.com/ant-cave/deep-student/commit/191884c5db7caaeef24790776735a103073e3722))
* **input-bar:** add QueuedMessageBubble/QueueErrorBar/QueuedMessageStack components ([f2f5c43](https://github.com/ant-cave/deep-student/commit/f2f5c4364b55335b9c3ae82a552707dc8f7d06d0))
* **input-bar:** add QueuedMessageBubble/QueueErrorBar/QueuedMessageStack components ([51b119e](https://github.com/ant-cave/deep-student/commit/51b119e4178f16887f656430dd1e1adae0b04bc7))
* **input-bar:** allow typing during streaming with queue, disable send at cap ([7eb5dba](https://github.com/ant-cave/deep-student/commit/7eb5dba8d401b2ac5ed8cb4ea6bc019db81c4040))
* **input-bar:** allow typing during streaming with queue, disable send at cap ([de74129](https://github.com/ant-cave/deep-student/commit/de741296a12699caa5fb8681a1252d717e0ff665))
* **input-bar:** mount QueuedMessageStack and wire queue settings ([16a0c4a](https://github.com/ant-cave/deep-student/commit/16a0c4a6a0713da9c4182b5315a723b0179e1a6f))
* **input-bar:** mount QueuedMessageStack and wire queue settings ([bd86c46](https://github.com/ant-cave/deep-student/commit/bd86c4636ce39ba6533a80d60c89670d1d5e0a2b))
* **input-bar:** route submit to enqueueMessage when streaming ([f55d592](https://github.com/ant-cave/deep-student/commit/f55d5928065f19c0ebfde87c03546f50db1576e2))
* **input-bar:** route submit to enqueueMessage when streaming ([a60c66c](https://github.com/ant-cave/deep-student/commit/a60c66c859f8ba92454cd9ffd38b06be1490f089))
* **layout:** improve mobile layout components and App entry ([04f75f7](https://github.com/ant-cave/deep-student/commit/04f75f730c3750719409ac7269e2b7b6954de1ae))
* **layout:** improve mobile layout components and App entry ([c3e909e](https://github.com/ant-cave/deep-student/commit/c3e909e09a58e7e0d1315ea65b356806f2f89de2))
* **locale:** update localization files for improved user experience ([7fafbc2](https://github.com/ant-cave/deep-student/commit/7fafbc29742c63d2e5d3faa85cece49709f8f897))
* **native-feel:** Phase A — chrome not selectable by default ([edd6b59](https://github.com/ant-cave/deep-student/commit/edd6b591945fbee980eb5f3b49d7f9d5c4ce32d5))
* **native-feel:** Phase A — chrome not selectable by default ([e64683d](https://github.com/ant-cave/deep-student/commit/e64683daee7c3dee2aa54021050b2d7879f92627))
* **native-feel:** Phase B — global contextmenu interception ([facb093](https://github.com/ant-cave/deep-student/commit/facb0931cb9dfb4f9e7ed27ae1bd274317b83761))
* **native-feel:** Phase B — global contextmenu interception ([1acd189](https://github.com/ant-cave/deep-student/commit/1acd18921ac27004c9b8361909e80125db43afe3))
* **native-feel:** Phase C — macOS-style overlay scrollbars ([5c62627](https://github.com/ant-cave/deep-student/commit/5c626277474e56837929d344ec317bec9fdd0b59))
* **native-feel:** Phase C — macOS-style overlay scrollbars ([0af9bb8](https://github.com/ant-cave/deep-student/commit/0af9bb82d7b1bba7bd3e4ab269d44ef3058e68c9))
* **native-feel:** Phase D1 — remove Topbar in-window drag widget ([48337d2](https://github.com/ant-cave/deep-student/commit/48337d214d31ce5f15bc1177156c24a13b1138d8))
* **native-feel:** Phase D1 — remove Topbar in-window drag widget ([d283356](https://github.com/ant-cave/deep-student/commit/d283356b9dd8e9380ab8098cf09f705c8f3e276d))
* **native-feel:** Phase D2 — macOS native menu bar ([f282746](https://github.com/ant-cave/deep-student/commit/f2827467ca80779d3d490f591d8e15127b5be02f))
* **native-feel:** Phase D2 — macOS native menu bar ([2c6e883](https://github.com/ant-cave/deep-student/commit/2c6e883d50a10618e538a24d15fc536407d8253f))
* **native-feel:** Phase D4 — focus rings, hover, transitions ([c79eb04](https://github.com/ant-cave/deep-student/commit/c79eb0409ee7c09ef7ddd4a6ada4edcd6e4d2ad2))
* **native-feel:** Phase D4 — focus rings, hover, transitions ([04fcc9e](https://github.com/ant-cave/deep-student/commit/04fcc9eb968f393727a6ed93eccc3498031d32c0))
* **native-feel:** Phase E — cursor strategy ([677250e](https://github.com/ant-cave/deep-student/commit/677250ed1ddf5b73aae8ef9663772f13d83add3f))
* **native-feel:** Phase E — cursor strategy ([ae9c551](https://github.com/ant-cave/deep-student/commit/ae9c55170e9b0ea5f829b7b460b0b27fa22360bd))
* **playground:** add eval framework, profiler, and enhanced mock adapter ([9441fdb](https://github.com/ant-cave/deep-student/commit/9441fdb701d09b8d3a4a4453224dbcc8e713b824))
* **playground:** add eval framework, profiler, and enhanced mock adapter ([c5800f1](https://github.com/ant-cave/deep-student/commit/c5800f1254beeda47ca7d87e44c3ed540feee69e))
* **pomodoro:** add daily goal feature with notifications and progress tracking ([c5780ed](https://github.com/ant-cave/deep-student/commit/c5780edefa517030621e63c97889f77cacfe17de))
* rebase v0.9.35 audit fixes onto nightly (Wave 1-7 reduced) ([88e5182](https://github.com/ant-cave/deep-student/commit/88e5182da7a078590337aa1d1a8954729343fa8f))
* rebase v0.9.35 audit fixes onto nightly (Wave 1-7 reduced) ([ea14191](https://github.com/ant-cave/deep-student/commit/ea14191a7f022c227ff84259476f89caf92af6ed))
* **sandbox:** add HTML sandbox preview and workbench ([43465f7](https://github.com/ant-cave/deep-student/commit/43465f740b49ba296c8bb4ca3c3a80276512a3fa))
* **sandbox:** add HTML sandbox preview and workbench ([4f197d4](https://github.com/ant-cave/deep-student/commit/4f197d48bee8d8fb321c3dacfaf7be10af949187))
* **settings:** add loading-safe toggles and thinking defaults ([502ff0c](https://github.com/ant-cave/deep-student/commit/502ff0cf2593d856a72cd05fc3dce28cea9e254e))
* **settings:** add loading-safe toggles and thinking defaults ([b8e5a56](https://github.com/ant-cave/deep-student/commit/b8e5a56400b870628abdd9d5610aea11c7e2482a))
* **settings:** enhance ModelsTab & vendor management ([310c3e9](https://github.com/ant-cave/deep-student/commit/310c3e90a83e711a5ac4349e462a9747c53473df))
* **settings:** enhance ModelsTab & vendor management ([553654e](https://github.com/ant-cave/deep-student/commit/553654e1bad806e7cfcdb0aa5e497838e3e00ce8))
* **settings:** enhance vendor management, API key UX & model converters ([17973f7](https://github.com/ant-cave/deep-student/commit/17973f7173f487643509920ad268263f3d902e77))
* **settings:** enhance vendor management, API key UX & model converters ([06cec4e](https://github.com/ant-cave/deep-student/commit/06cec4ea7702c2dcba8ab5563c54cd6b9af10b46))
* **settings:** group provider models by family with sticky headers ([57bc4b6](https://github.com/ant-cave/deep-student/commit/57bc4b686227bb9f93de43526a7ecb0be714aa78))
* **settings:** group provider models by family with sticky headers ([5d872f2](https://github.com/ant-cave/deep-student/commit/5d872f234cac11fcecee492d0fcb2ebaa479226f))
* **settings:** migrate panels to study UI tokens ([5583726](https://github.com/ant-cave/deep-student/commit/558372654786e05a6ebd76258ab218b0ea2afb86))
* **settings:** migrate panels to study UI tokens ([dd7b003](https://github.com/ant-cave/deep-student/commit/dd7b003d49edf8910b7a98aa9184d349546f6434))
* **settings:** refresh About — channel Select + acknowledgements layout ([8f8d2d1](https://github.com/ant-cave/deep-student/commit/8f8d2d165e753db7cbf83b205d86dc6133dc0fe6))
* **settings:** refresh About — channel Select + acknowledgements layout ([c9b2526](https://github.com/ant-cave/deep-student/commit/c9b2526c2161539b685bc3eafe3cd7eb3bab4529))
* **settings:** update AppearanceTab, i18n locales, and CSS styles ([3687b93](https://github.com/ant-cave/deep-student/commit/3687b9308cf7fb8247f7eeed55b897538abd18b4))
* **settings:** update AppearanceTab, i18n locales, and CSS styles ([32c4f4e](https://github.com/ant-cave/deep-student/commit/32c4f4e7421f1d0a0302f55c3f882c580b9bf185))
* **shell:** integrate todo sidebar and tighten header nav routing ([75b7cf4](https://github.com/ant-cave/deep-student/commit/75b7cf40b0140e714a2757b2a3bb61b4707d2695))
* **shell:** integrate todo sidebar and tighten header nav routing ([9caeda0](https://github.com/ant-cave/deep-student/commit/9caeda01c9e980b75c90d8dfd7a7472d60eb945b))
* **streaming:** rAF-based smoothing engine with commit gate and word boundary support ([50ca33a](https://github.com/ant-cave/deep-student/commit/50ca33ab2dc9220d6da284efa1dba40c38448366))
* **streaming:** rAF-based smoothing engine with commit gate and word boundary support ([91d5255](https://github.com/ant-cave/deep-student/commit/91d5255588969c0f0bcfce8619b6fa8cae9c451b))
* **style-lab:** add Segmented/Select compare sections and update tests ([16ff3c3](https://github.com/ant-cave/deep-student/commit/16ff3c38b0ea2a78785241c247e2a6e3ddd0497c))
* **style-lab:** add Segmented/Select compare sections and update tests ([9f0a227](https://github.com/ant-cave/deep-student/commit/9f0a227eb9cfc547448e227861324ef11d810821))
* **style-lab:** expand ComponentCompareTab with new preview panels ([08371a6](https://github.com/ant-cave/deep-student/commit/08371a699d76efbb0c8f9f996a1a3019252cbe96))
* **style-lab:** expand ComponentCompareTab with new preview panels ([f29fed7](https://github.com/ant-cave/deep-student/commit/f29fed7d3d73457264770ee821636e388f788579))
* **sync:** 添加 todo_items 分类到同步分类注册表并更新测试 ([ab6fa4e](https://github.com/ant-cave/deep-student/commit/ab6fa4eec0d2b21196ecea38c3b1199c3a722e4e))
* **sync:** 添加 todo_items 分类到同步分类注册表并更新测试 ([e600d53](https://github.com/ant-cave/deep-student/commit/e600d53aa96901363beb38ba8ec8f84ee488e141))
* **todo:** add recycle bin functionality with restore and purge options ([850c2f3](https://github.com/ant-cave/deep-student/commit/850c2f39d57bc97c56e26996433ab7e7c590b6c9))
* **todo:** enhance error handling and notifications in todo store ([7fafbc2](https://github.com/ant-cave/deep-student/commit/7fafbc29742c63d2e5d3faa85cece49709f8f897))
* **tokens:** establish 3-layer design token system (primitives → semantic → [@theme](https://github.com/theme)) ([795ac32](https://github.com/ant-cave/deep-student/commit/795ac3245c89b022d2b2076c145faab26608cbde))
* **tokens:** establish 3-layer design token system (primitives → semantic → [@theme](https://github.com/theme)) ([1b1b80f](https://github.com/ant-cave/deep-student/commit/1b1b80f1c0119afef920f804c7811682ae572d08))
* **translation:** add chat_popover translation endpoint & wire backend config ([07fe6f6](https://github.com/ant-cave/deep-student/commit/07fe6f60eed02d5fad6bcc3e7e3315607760792a))
* **translation:** add chat_popover translation endpoint & wire backend config ([4a1e4a4](https://github.com/ant-cave/deep-student/commit/4a1e4a4e96cfe6a4afbc77eae0d759b1bde7cb8a))
* **ui:** add iOS-style sliding thumb to SegmentedControl ([1135de6](https://github.com/ant-cave/deep-student/commit/1135de6f4f6430f188c468f1e5988d3c4a6cd58d))
* **ui:** add iOS-style sliding thumb to SegmentedControl ([f0a82e2](https://github.com/ant-cave/deep-student/commit/f0a82e24fd0b76c54e485a24a74a68649fde544c))
* **ui:** add shared inputShell, Select component, and align Input/Textarea ([d3da6fc](https://github.com/ant-cave/deep-student/commit/d3da6fcd13f8f0387221fe6872252bb956da7784))
* **ui:** add shared inputShell, Select component, and align Input/Textarea ([694bf13](https://github.com/ant-cave/deep-student/commit/694bf135d1680e067ce1e2526691213f122a9eda))
* **ui:** adopt shared scroll and overlay primitives ([49b88b9](https://github.com/ant-cave/deep-student/commit/49b88b95c7c021c8d36a07239558ee52b7fa6ca0))
* **ui:** adopt shared scroll and overlay primitives ([d90c043](https://github.com/ant-cave/deep-student/commit/d90c0432d0bc47391dd3cfb7a911b587a978a3ec))
* **ui:** gate UI Lab view behind debug-panel toggle ([c2e7cae](https://github.com/ant-cave/deep-student/commit/c2e7cae8428d858fef2831755097f5c931a0f1fc))
* **ui:** gate UI Lab view behind debug-panel toggle ([1c44898](https://github.com/ant-cave/deep-student/commit/1c4489862a50a4a064082ff86057e9ae15ca6cc5))
* update chat workflows and settings UX ([1fcfca6](https://github.com/ant-cave/deep-student/commit/1fcfca6d9695b50377930e59060cee2e42e36a1c))


### Bug Fixes

* add @lobehub/ui and antd dependencies ([8678880](https://github.com/ant-cave/deep-student/commit/86788807e07bcabd4ac1335b39409ede7a54c962))
* add @lobehub/ui and antd dependencies ([c2f43f8](https://github.com/ant-cave/deep-student/commit/c2f43f8bfd16624491b2ba4d9bc892ffc9515142))
* **anki,skills:** unify mobile page header navigation config ([#110](https://github.com/ant-cave/deep-student/issues/110)) ([dd354ef](https://github.com/ant-cave/deep-student/commit/dd354efd8a8782676e299541bdc8a39efdabc9db))
* **app:** tighten create navigation and note flushing ([6f6574e](https://github.com/ant-cave/deep-student/commit/6f6574e41a4d98cd95ae492b1f21418b876fdac4))
* **app:** tighten create navigation and note flushing ([62b978a](https://github.com/ant-cave/deep-student/commit/62b978a978c31599d8dc4ba1a7de5010a2ec1a58))
* automated recovery of all missing CSS rules from original files ([774700b](https://github.com/ant-cave/deep-student/commit/774700b58cdb7606a260d100dcb6ca93fd5e0f33))
* automated recovery of all missing CSS rules from original files ([9b79239](https://github.com/ant-cave/deep-student/commit/9b792399053888bf90929ecca8dfdb2d25a66c38))
* backend defect batch — crypto rotation, migration guards, paths, chat_v2, anki, vfs resume ([8ed2116](https://github.com/ant-cave/deep-student/commit/8ed21167dfaf021800503b3721d375a19ecb59d3))
* **backend:** normalize tool protocols and governance fallback ([06223ef](https://github.com/ant-cave/deep-student/commit/06223efc527b2969da972b8391558abbdacefcbd))
* **backend:** normalize tool protocols and governance fallback ([e4368ae](https://github.com/ant-cave/deep-student/commit/e4368ae2a0166c9726848c12391c22ca9ad0c571))
* **build:** refresh lockfile after nightly rebase ([d860d07](https://github.com/ant-cave/deep-student/commit/d860d07cf87bb6f37421bd855d815c834d7773b0))
* **build:** refresh lockfile after nightly rebase ([9253d5d](https://github.com/ant-cave/deep-student/commit/9253d5d5327e639ef81b825fc52e71401e851f1e))
* **build:** refresh rust lockfile ([09c5587](https://github.com/ant-cave/deep-student/commit/09c55878ea2c327d234499d144671d94ec39f177))
* **build:** refresh rust lockfile ([acac391](https://github.com/ant-cave/deep-student/commit/acac3913f8686cf48d2b1d23b787b54e3e948b9e))
* **build:** regenerate package-lock.json to sync with package.json ([c5f64e7](https://github.com/ant-cave/deep-student/commit/c5f64e750bf92e4080aa04c51504567c4ed1bd26))
* **build:** regenerate package-lock.json to sync with package.json ([e265bcd](https://github.com/ant-cave/deep-student/commit/e265bcd34ecb028fea9f3db77a390df76afa6b96))
* **build:** resolve white screen - remove circular vendor chunks and barrel imports ([4ab4838](https://github.com/ant-cave/deep-student/commit/4ab48384eed4db2e57f8919f3e86a398c1ff5ea0))
* **build:** resolve white screen - remove circular vendor chunks and barrel imports ([c4d1056](https://github.com/ant-cave/deep-student/commit/c4d1056736a1fd5b1e4f2e36dae0b353c06f9e97))
* **build:** restore ppt-rs@0.2 (non-yanked) + suppress yanked in cargo-audit ([47145bb](https://github.com/ant-cave/deep-student/commit/47145bb922f7f2e29d90ab5c2932894b578a41fc))
* **build:** restore ppt-rs@0.2 (non-yanked) + suppress yanked in cargo-audit ([87abb82](https://github.com/ant-cave/deep-student/commit/87abb8214f7b8a7dd141092ff49deb34e333a5f5))
* **chat-store:** add queuedMessages/dequeuing to ChatStore interface ([c7a4016](https://github.com/ant-cave/deep-student/commit/c7a401607bdf846061f17e3446468b4e0d822a1b))
* **chat-store:** add queuedMessages/dequeuing to ChatStore interface ([2228713](https://github.com/ant-cave/deep-student/commit/2228713e4a821dfaf0fe321c71ad128c6f18460c))
* **chat-store:** tolerant blocking read + breather self-retry + ordered guard ([27d2921](https://github.com/ant-cave/deep-student/commit/27d29214a02d1ced4f76761cc4474217995a31ea))
* **chat-store:** tolerant blocking read + breather self-retry + ordered guard ([f261c31](https://github.com/ant-cave/deep-student/commit/f261c318ff1b1dcec04a3040ee4b61e1bbc6858c))
* **chat-store:** tolerate both pendingBlockingInteraction and pendingApprovalRequest in dequeue heartbeat ([7bdb0c5](https://github.com/ant-cave/deep-student/commit/7bdb0c58b344ae900a273fbc78491e1bcdc98d80))
* **chat-store:** tolerate both pendingBlockingInteraction and pendingApprovalRequest in dequeue heartbeat ([1bec8ee](https://github.com/ant-cave/deep-student/commit/1bec8ee59f4c548d993d39e8e22bd1f1d12dc2fc))
* **chat/thinking:** unify font-size to --font-size-sm token, drop !important ([c85d4fd](https://github.com/ant-cave/deep-student/commit/c85d4fd6bef0d993de93893dac267330251eb982))
* **chat/thinking:** unify font-size to --font-size-sm token, drop !important ([665a5ca](https://github.com/ant-cave/deep-student/commit/665a5caa0075aaa47512c1142a1ac245e3e9aeae))
* **ci:** align workflow installs with release dependency constraints ([cd80106](https://github.com/ant-cave/deep-student/commit/cd8010687445c897df789a8f08c8c42f828bf057))
* **ci:** align workflow installs with release dependency constraints ([4720285](https://github.com/ant-cave/deep-student/commit/4720285f8571ea56ec771b40599af63160a3ab88))
* **ci:** ignore 3 aws-lc-sys RUSTSEC advisories (transitive via aws-sdk-s3) ([00ce57e](https://github.com/ant-cave/deep-student/commit/00ce57ed7fb55001306fbe6b76f451b146700ffd))
* **ci:** ignore 3 aws-lc-sys RUSTSEC advisories (transitive via aws-sdk-s3) ([d82f411](https://github.com/ant-cave/deep-student/commit/d82f4118bb62648de24a62a325015fa3d00d6559))
* **ci:** make cargo audit informational (non-blocking) ([2762a18](https://github.com/ant-cave/deep-student/commit/2762a18bedfb2dfc107c7a128d6a7474be107752))
* **ci:** make cargo audit informational (non-blocking) ([32d4556](https://github.com/ant-cave/deep-student/commit/32d45561532beeda4d2f5a96ad5c19986f6d5755))
* **ci:** resolve PR [#67](https://github.com/ant-cave/deep-student/issues/67) build failures ([847b802](https://github.com/ant-cave/deep-student/commit/847b802fbfc80fe6771e8ba0a0130f19d0e30eb3))
* **ci:** resolve PR [#67](https://github.com/ant-cave/deep-student/issues/67) build failures ([c0cabf4](https://github.com/ant-cave/deep-student/commit/c0cabf4b0ed745429c19a470ebcf9e35e234a158))
* **ci:** revert to cargo audit (no --deny-warnings, ppt-rs removal fixes 2/3 yanked crates) ([d538eff](https://github.com/ant-cave/deep-student/commit/d538eff659f653fc1dd3a33fb02c78823ba118af))
* **ci:** revert to cargo audit (no --deny-warnings, ppt-rs removal fixes 2/3 yanked crates) ([8db821b](https://github.com/ant-cave/deep-student/commit/8db821b04e72f422b0abc137b698f9cd1a11ef9a))
* **ci:** split sync regression targets across jobs ([4a55c40](https://github.com/ant-cave/deep-student/commit/4a55c40dceca6a5f8b5b9b8ed4e5744d05560230))
* **ci:** split sync regression targets across jobs ([38390f9](https://github.com/ant-cave/deep-student/commit/38390f9a2f7e89fd5099012cd8cff6783928b6d2))
* **ci:** use --legacy-peer-deps for @lobehub/icons React 19 peer conflict ([18bb47f](https://github.com/ant-cave/deep-student/commit/18bb47f9b4d48f6ca196404db156c2d69b100ea0))
* **ci:** use --legacy-peer-deps for @lobehub/icons React 19 peer conflict ([aaeece3](https://github.com/ant-cave/deep-student/commit/aaeece3fedf960cdd16f79364e359a5ec4c16598))
* clean review fallout before push ([6ffad9e](https://github.com/ant-cave/deep-student/commit/6ffad9ea68872589900dbcc0f11ff7fe60c0c9c8))
* extract all missing DeepStudent.css styles into proper locations ([40d7dc8](https://github.com/ant-cave/deep-student/commit/40d7dc8daa117b5999d59de391f32dbffff1880e))
* extract all missing DeepStudent.css styles into proper locations ([51e0858](https://github.com/ant-cave/deep-student/commit/51e0858d67366d80fb2050bc3d343873c8ba5bef))
* **hooks:** persist last question position in question bank session ([7fafbc2](https://github.com/ant-cave/deep-student/commit/7fafbc29742c63d2e5d3faa85cece49709f8f897))
* **input-bar:** defensive default for queuedMessages in selectors ([19a1f06](https://github.com/ant-cave/deep-student/commit/19a1f06c4bccbdad518fb82b0a7bba2e590ae797))
* **input-bar:** defensive default for queuedMessages in selectors ([4b279ae](https://github.com/ant-cave/deep-student/commit/4b279aea387b17aa9d131ae119cb3de82218729f))
* **merge:** keep nightly dependency baseline ([77b40f4](https://github.com/ant-cave/deep-student/commit/77b40f4992ba0ae40a4ae4e723258d851d4e25af))
* **merge:** keep nightly dependency baseline ([cb4d001](https://github.com/ant-cave/deep-student/commit/cb4d0017163a468f6c3fbb1f1cb8affb67e0f65b))
* minor fixes across learning-hub and notes sidebar components ([72613ec](https://github.com/ant-cave/deep-student/commit/72613ecfa351b7ebbecc1d5ac91d23f00a233036))
* minor fixes across learning-hub and notes sidebar components ([6c1fe34](https://github.com/ant-cave/deep-student/commit/6c1fe34ece5ddcad1a115a20f6528a87a7d63c9f))
* pin @lobehub/icons to 5.6.0 ([e6f20b1](https://github.com/ant-cave/deep-student/commit/e6f20b183e8621618962a8ea12174565fb94221c))
* pin @lobehub/icons to 5.6.0 ([d04fb13](https://github.com/ant-cave/deep-student/commit/d04fb132ec29b081b93057cf20d11d750b130ebf))
* pin @lobehub/icons to 5.6.0 (5.8.0 causes React 19 runtime incompatibility) ([4fb3482](https://github.com/ant-cave/deep-student/commit/4fb3482d547f3ab50791c453db7c0761ff010f4e))
* pin @lobehub/icons to 5.6.0 (5.8.0 causes React 19 runtime incompatibility) ([b38c81d](https://github.com/ant-cave/deep-student/commit/b38c81dab7a911575ce5f0097007cea6249bd216))
* preserve cloud sync conflicts before strategy filtering ([c0fd61e](https://github.com/ant-cave/deep-student/commit/c0fd61ea2715c0c7de5a419c6a8be8309f7e50e4))
* **rebuild:** add --legacy-peer-deps to npm ci ([e2aa940](https://github.com/ant-cave/deep-student/commit/e2aa940fa6172252ec0c446877ff178fd871b974))
* **rebuild:** add --legacy-peer-deps to npm ci ([449a0c2](https://github.com/ant-cave/deep-student/commit/449a0c2a71cdc0411e18dddaa98c62a757513724))
* recover all missing sidebar/shell/layout styles from originals ([c5aa928](https://github.com/ant-cave/deep-student/commit/c5aa928d8d60843d78a1b3e08bb5cb2d1e67c617))
* recover all missing sidebar/shell/layout styles from originals ([cde8038](https://github.com/ant-cave/deep-student/commit/cde803819c10b8e0dc4305f141166446a6baabd9))
* **release:** add --legacy-peer-deps to npm ci ([1f98c0d](https://github.com/ant-cave/deep-student/commit/1f98c0deed0d8f6bacb4abb0f585497686220003))
* **release:** add --legacy-peer-deps to npm ci ([e0bb680](https://github.com/ant-cave/deep-student/commit/e0bb680f32b451127962713f83a6641a4bbef371))
* **release:** add --legacy-peer-deps to npm ci ([8a9189b](https://github.com/ant-cave/deep-student/commit/8a9189b7a55e52eb39d8ccdd04b32ec5d05ef16e))
* **release:** add --legacy-peer-deps to npm ci ([bc0603f](https://github.com/ant-cave/deep-student/commit/bc0603fd6db1225abb7b3405e402bf68a21125b7))
* remove conflicting token imports from tailwind.css ([fab9248](https://github.com/ant-cave/deep-student/commit/fab92483eafb95dc6b90eb2f2ec960cd0edfcfa8))
* remove conflicting token imports from tailwind.css ([5d72342](https://github.com/ant-cave/deep-student/commit/5d723427ed3dfd2f7d26306795ff7f1c87bde55a))
* repair build blockers and state typings ([86f05c2](https://github.com/ant-cave/deep-student/commit/86f05c27c9d1cc108e2a7dc095581f22f033e796))
* repair build blockers and state typings ([6706428](https://github.com/ant-cave/deep-student/commit/6706428a4457fb14425d2a2c00955f0d8f316478))
* restore broken CSS import chains (chat init, settings api-config) ([ed0c832](https://github.com/ant-cave/deep-student/commit/ed0c832178293ba4b9b0a48520e2ada706fe21af))
* restore broken CSS import chains (chat init, settings api-config) ([af0af27](https://github.com/ant-cave/deep-student/commit/af0af27445af213c4cb3a8394b4ff2e39030f3af))
* restore legacy CSS as fallback to recover missing styles ([33cf891](https://github.com/ant-cave/deep-student/commit/33cf891f9f74307e1850a459c7338eafecefb70d))
* restore legacy CSS as fallback to recover missing styles ([6f9584d](https://github.com/ant-cave/deep-student/commit/6f9584d8c107bf14d2abe446dc62053da26981a3))
* restore missing sidebar/shell styles from DeepStudent.css ([6782f0a](https://github.com/ant-cave/deep-student/commit/6782f0ab25e5a2a833d605f433e4a1be80d3e9a8))
* restore missing sidebar/shell styles from DeepStudent.css ([dc1b8e3](https://github.com/ant-cave/deep-student/commit/dc1b8e38f64e7c33d89ad265fed4f96aa5f2f744))
* restore original App.css + DeepStudent.css intact — stop decomposition ([2ad71c4](https://github.com/ant-cave/deep-student/commit/2ad71c49deaa1da91b24f6b0b12716808fdd1039))
* restore original App.css + DeepStudent.css intact — stop decomposition ([9717c95](https://github.com/ant-cave/deep-student/commit/9717c95158e16b279eef12b693a8cbaf42bbe519))
* review-driven defect sweep — anki idempotency, sync convergence, checker boundaries ([aa14a5e](https://github.com/ant-cave/deep-student/commit/aa14a5e2bd7399320c1edf77635055ef96c0e96a))
* round 2 verification — restore variant sanitization + drop 3 orphan modules ([d747822](https://github.com/ant-cave/deep-student/commit/d747822f7ea0dd8661fb297b9da15f5efb0e3ead))
* round 2 verification — restore variant sanitization + drop 3 orphan modules ([92ec7f3](https://github.com/ant-cave/deep-student/commit/92ec7f39f9722f264670d466ec4bce5af42a6fc6))
* **settings:** add placeholder for read-only models to align with deletable ones ([6556a8d](https://github.com/ant-cave/deep-student/commit/6556a8d01c2107c9b2b1200cfa8e1407bedce51e))
* **settings:** add placeholder for read-only models to align with deletable ones ([040934c](https://github.com/ant-cave/deep-student/commit/040934cb42346aa6fff611da3105536b54c9914f))
* **settings:** align model card actions by moving capability icons to right side ([2ef3314](https://github.com/ant-cave/deep-student/commit/2ef331464feda03566513a3914db2536212aecb9))
* **settings:** align model card actions by moving capability icons to right side ([4647ff0](https://github.com/ant-cave/deep-student/commit/4647ff08a319c7048652e8494c94d019ee651ebd))
* **settings:** make entire vendor row draggable, show grip icon on hover at right ([03ffea1](https://github.com/ant-cave/deep-student/commit/03ffea1d8618a69db42a298dc4dd806f26ee4533))
* **settings:** make entire vendor row draggable, show grip icon on hover at right ([f4a69bc](https://github.com/ant-cave/deep-student/commit/f4a69bc111c99d2dba577ca5c86545ff04349ddc))
* **settings:** optimistic reorder to prevent flash-back during persist ([c79be91](https://github.com/ant-cave/deep-student/commit/c79be91fce143df1c7db1ba3f0157c88e0af476e))
* **settings:** optimistic reorder to prevent flash-back during persist ([1cab5de](https://github.com/ant-cave/deep-student/commit/1cab5de7fb6b882894808a810f0ace5f96083e43))
* **settings:** remove drag offset hack that caused position jumping ([98ddb11](https://github.com/ant-cave/deep-student/commit/98ddb11571e482b173122e1730aeb36a52aaa290))
* **settings:** remove drag offset hack that caused position jumping ([adcd5cb](https://github.com/ant-cave/deep-student/commit/adcd5cb4c3ccfdf082128fb8edb8d71dce1fba7c))
* **settings:** remove recommended badge from vendor sidebar list ([6e2cdd2](https://github.com/ant-cave/deep-student/commit/6e2cdd2e4eecc62d5554656abd2758def97acf2c))
* **settings:** remove recommended badge from vendor sidebar list ([8f435ce](https://github.com/ant-cave/deep-student/commit/8f435ce8beae53cf79c90f1a39a0e8ae06a7fab5))
* **settings:** remove redundant divider between connection config and model section ([a3c2fe7](https://github.com/ant-cave/deep-student/commit/a3c2fe711e450dbeb561191ade72d3caa6915bce))
* **settings:** remove redundant divider between connection config and model section ([199e183](https://github.com/ant-cave/deep-student/commit/199e183ae1e1f044ca87a342de2ccbb5638a6446))
* **settings:** remove vendorBusy spinner that caused layout shift on reorder ([2a8c4d3](https://github.com/ant-cave/deep-student/commit/2a8c4d3d7c5461696897f9ecd1f4ec59ee04fe0b))
* **settings:** remove vendorBusy spinner that caused layout shift on reorder ([77e727e](https://github.com/ant-cave/deep-student/commit/77e727eb3f829408ba85b33cae3fb724fd5d806d))
* **settings:** show base URL field when creating new vendor for complete setup in one step ([78e095a](https://github.com/ant-cave/deep-student/commit/78e095a9198b6080f3baecbe0c4c32d318fa1da3))
* **settings:** show base URL field when creating new vendor for complete setup in one step ([6bced05](https://github.com/ant-cave/deep-student/commit/6bced058034db8df365bba94861da7fa4013dfda))
* **settings:** use renderClone to fix drag position drift in sticky sidebar ([d9c4c3a](https://github.com/ant-cave/deep-student/commit/d9c4c3a9c700f5adb61dd63f3eb58c2b86f250d2))
* **settings:** use renderClone to fix drag position drift in sticky sidebar ([bffd314](https://github.com/ant-cave/deep-student/commit/bffd3143530ad05362db52cdbc816776d1588e8a))
* **shell:** remove transition-colors on sidebar to prevent theme flash ([1239aaa](https://github.com/ant-cave/deep-student/commit/1239aaa9099cc4544a015aa5cde86b2675835046))
* **shell:** remove transition-colors on sidebar to prevent theme flash ([7ee3566](https://github.com/ant-cave/deep-student/commit/7ee3566595bd1307aa5530e3b5b64ba86ee58c15))
* **sidebar:** definitive sidebar CSS recovery — single source of truth ([e77c8d3](https://github.com/ant-cave/deep-student/commit/e77c8d358673b2b57e3a6edd1d7754ae0cef0a6f))
* **sidebar:** definitive sidebar CSS recovery — single source of truth ([b26fee6](https://github.com/ant-cave/deep-student/commit/b26fee6235212f9a1809bb2bdd88a03491d060fa))
* **sync:** restore baseline without creating local drift ([1a3ed99](https://github.com/ant-cave/deep-student/commit/1a3ed9962bc1ea7e34d997c591cd38dd6b7dd5a7))
* **sync:** restore baseline without creating local drift ([66b1ac7](https://github.com/ant-cave/deep-student/commit/66b1ac78e2a0f764b85c4c2afbb7efc931169970))
* **ts:** resolve 6 pre-existing TypeScript errors from nightly ([3a99340](https://github.com/ant-cave/deep-student/commit/3a99340ac5f587293071abccf88509937bb14561))
* **ts:** resolve 6 pre-existing TypeScript errors from nightly ([a3e7966](https://github.com/ant-cave/deep-student/commit/a3e7966f4dde31e5ceb2a641b3120e99047f0dc8))
* **ui:** raise Select dropdown z-index above modal overlay ([c1e19d8](https://github.com/ant-cave/deep-student/commit/c1e19d8b6e40a76322204ed53295f0b4d5f1b0f4))
* **ui:** raise Select dropdown z-index above modal overlay ([8244ec5](https://github.com/ant-cave/deep-student/commit/8244ec5d6d6b9ecd685cde84d72f4478ded54d11))
* update App.css to use explicit [@import](https://github.com/import) for cross-platform compatibility and adjust versionCode in tauri.conf.json ([b43b144](https://github.com/ant-cave/deep-student/commit/b43b14413356284317e6718c1f0504ab903c5b81))
* 补充canonical_tools.rs注册 ([1c87216](https://github.com/ant-cave/deep-student/commit/1c8721674af9dbd0f338c4e24c02591704db79d8))


### Performance Improvements

* **chat/timeline:** memoize paragraph split & cap thinking body to scrollable max-height ([e20c25e](https://github.com/ant-cave/deep-student/commit/e20c25e3def47bbd8143efb109b0856bcf2955f2))
* **chat/timeline:** memoize paragraph split & cap thinking body to scrollable max-height ([bf645c7](https://github.com/ant-cave/deep-student/commit/bf645c71902216d813b3621002c4b78a38177688))


### Reverts

* rollback Tailwind v4 to v3 — CSS layer conflicts with existing styles ([fab81e4](https://github.com/ant-cave/deep-student/commit/fab81e40c6a92c0c975e8312fb2b829024901107))
* rollback Tailwind v4 to v3 — CSS layer conflicts with existing styles ([742961a](https://github.com/ant-cave/deep-student/commit/742961ab68cb6372f305e6c1b0e59f8e8114e714))

## [0.9.38](https://github.com/helixnow/deep-student/compare/v0.9.37...v0.9.38) (2026-05-24)


### Bug Fixes

* add @lobehub/ui and antd dependencies ([c2f43f8](https://github.com/helixnow/deep-student/commit/c2f43f8bfd16624491b2ba4d9bc892ffc9515142))

## [0.9.37](https://github.com/helixnow/deep-student/compare/v0.9.36...v0.9.37) (2026-05-24)


### Bug Fixes

* pin @lobehub/icons to 5.6.0 ([d04fb13](https://github.com/helixnow/deep-student/commit/d04fb132ec29b081b93057cf20d11d750b130ebf))
* **rebuild:** add --legacy-peer-deps to npm ci ([449a0c2](https://github.com/helixnow/deep-student/commit/449a0c2a71cdc0411e18dddaa98c62a757513724))
* **release:** add --legacy-peer-deps to npm ci ([e0bb680](https://github.com/helixnow/deep-student/commit/e0bb680f32b451127962713f83a6641a4bbef371))

## [0.9.36](https://github.com/helixnow/deep-student/compare/v0.9.35...v0.9.36) (2026-05-24)


### Features

* **data_governance:** support virtual URI targets for ZIP exports ([b5bd171](https://github.com/helixnow/deep-student/commit/b5bd171fb5a8c16f71797c5bf191c5e25e31a320))


### Bug Fixes

* 修正在学习资源内题库中答题结束的祝贺弹窗在移动端的错误位置 ([#51](https://github.com/helixnow/deep-student/issues/51)) ([f6690e9](https://github.com/helixnow/deep-student/commit/f6690e960585f0338d96b95146479ec3566c036b))

## [0.9.35](https://github.com/helixnow/deep-student/compare/v0.9.34...v0.9.35) (2026-03-14)


### Features

* **todo:** add database constraints and improve code formatting ([2500b9c](https://github.com/helixnow/deep-student/commit/2500b9ce34550b131eeb3775da7658c74bd211d9))
* **tools:** add arg_utils for JSON parsing and MCP server configuration ([44c70b4](https://github.com/helixnow/deep-student/commit/44c70b4570bffbd087c573ee9be2c37dd1940542))


### Bug Fixes

* **ci:** auto-recover android release builds ([ac74c9b](https://github.com/helixnow/deep-student/commit/ac74c9be414f2a4b61f22224cfccec7b6d2cf829))
* **ci:** avoid android rebuild invalidation and add heartbeat ([4740877](https://github.com/helixnow/deep-student/commit/4740877946eafdabf55c6382c440e4f5be1391e3))
* **ci:** remove android tee wrapper and add timeout ([79df4e0](https://github.com/helixnow/deep-student/commit/79df4e0813b5b3ee405105bda57e45fe96e1b097))
* **ci:** retry transient android dependency failures ([3734c5c](https://github.com/helixnow/deep-student/commit/3734c5ce3c5612d6dc65c2cedee84d72da6a88f0))

## [0.9.34](https://github.com/helixnow/deep-student/compare/v0.9.33...v0.9.34) (2026-03-09)


### Features

* **i18n:** add Todo localization support for en-US and zh-CN ([e61ee8e](https://github.com/helixnow/deep-student/commit/e61ee8e561e99ca44fe7b57bac283ad7eaa35494))
* **pomodoro:** add immersive focus mode with white noise and circular progress ([2ee581c](https://github.com/helixnow/deep-student/commit/2ee581cc41cc55f2053811e414a27310c872d7e0))
* **pomodoro:** add Pomodoro timer support for todo items ([6ad54d9](https://github.com/helixnow/deep-student/commit/6ad54d9765f0e7bd7c903525672cd1ba724c3ae8))
* **todo:** add comprehensive Todo support across DSTU system ([3863cf9](https://github.com/helixnow/deep-student/commit/3863cf9384fc327dc6b27a089e8438ca4f1a61db))
* **todo:** add Todo resource type support across Learning Hub ([b8e418d](https://github.com/helixnow/deep-student/commit/b8e418dd7225cf48c549c0ed419918da065bb21d))
* **vfs:** decouple todo_lists from VFS resources system ([2be0e94](https://github.com/helixnow/deep-student/commit/2be0e943b263a0b544009c26f9b4a0121ff1cb4a))


### Bug Fixes

* **build:** bump Android versionCode to 13516 and add parse_timestamp import ([045703e](https://github.com/helixnow/deep-student/commit/045703ef5c454dcce0da62405fab03bc48b5dce2))
* **ci:** add three-path release detection to handle merge commits burying release commit ([466152c](https://github.com/helixnow/deep-student/commit/466152c651918718833dbf311a4307ed345fe4c6))
* **ci:** harden Android build against runner resource exhaustion ([985bc7b](https://github.com/helixnow/deep-student/commit/985bc7bc9f7ad4ced66d5d97e56fad3248024ec5))
* **settings:** prevent auto-save from overwriting backend config when loadConfig fails ([21fbb00](https://github.com/helixnow/deep-student/commit/21fbb00106e6408e8948f171e78153040fdeab39))


### Performance Improvements

* **bundle:** optimize initial load performance with lazy loading and selective subscriptions ([0da3cba](https://github.com/helixnow/deep-student/commit/0da3cbab0f0d3a1b7ebd8315d3354c1c31f88d83))

## [0.9.33](https://github.com/helixnow/deep-student/compare/v0.9.32...v0.9.33) (2026-03-08)


### Features

* **llm:** add model capability registry with automatic vision/tools/reasoning inference ([837aa6c](https://github.com/helixnow/deep-student/commit/837aa6ce338d2f9bbd20d98555906b93987249c1))
* **memory-system:** hide system-reserved folders/notes with `__*__` pattern across Finder and implement memory folder navigation ([7ddf4c3](https://github.com/helixnow/deep-student/commit/7ddf4c3c743a581f25ef72ba36afd3973e8b98f7))
* **notes,textbooks:** detect and sanitize opaque Android document IDs in filenames across frontend and backend ([d75ac97](https://github.com/helixnow/deep-student/commit/d75ac976eea724243ed1473bb54b3910fd681669))
* **notes,textbooks:** extract H1 heading from markdown when title is generic placeholder and generate friendly names for opaque document IDs ([c62a022](https://github.com/helixnow/deep-student/commit/c62a022aad40ac6da8f2567402038762bcee778a))
* **notes:** add reading mode toggle to prevent keyboard popup on mobile during scrolling ([648d763](https://github.com/helixnow/deep-student/commit/648d7636eea2d9d11d0f22effc8922351f91582e))
* **pdf,polyfills:** add Promise.withResolvers polyfill for older browsers and remove unused active feature chips ([aebf481](https://github.com/helixnow/deep-student/commit/aebf481d35dd6907e0f380df04c04f7ad6fc50ce))
* **question-bank:** add question history view and refactor timer management for advanced practice modes ([36746a8](https://github.com/helixnow/deep-student/commit/36746a8f124c82d93f61ec95c0723df7d27fdd41))
* **skills-executor:** add custom deserializer to handle stringified array parameters from LLMs ([493677f](https://github.com/helixnow/deep-student/commit/493677fe7145a27014ba358ec5ffc3f74969151a))
* **todo:** add user-facing todo system with database schema and system prompt integration ([ba1dfa4](https://github.com/helixnow/deep-student/commit/ba1dfa471a1c6ea49c640047e17a41525768a9e9))


### Bug Fixes

* **ci:** detect merged release commits with PR suffix ([14547bb](https://github.com/helixnow/deep-student/commit/14547bbc726bcabaf4960e2c085582f36b6cb35c))

## [0.9.32](https://github.com/helixnow/deep-student/compare/v0.9.31...v0.9.32) (2026-03-06)


### Features

* **chat_v2,workspace,qbank,sync:** add cross-session permission checks and harden tool whitelist bypass ([04a9b10](https://github.com/helixnow/deep-student/commit/04a9b10ac9b8a446f811dfc06b5915f386a0a956))
* **chat-v2,learning-hub:** enhance resource handling and state management ([168c253](https://github.com/helixnow/deep-student/commit/168c253780c9833c2fd0d6d3e19e63dbe76893f1))
* **chat-v2:** enhance skill state management and event handling ([3c8027a](https://github.com/helixnow/deep-student/commit/3c8027aaada91c74f8a98de4b3e915a504f1ffb2))
* **chat,vfs:** add answer submission idempotency and enhance context ref handling ([580db0f](https://github.com/helixnow/deep-student/commit/580db0f271f6ad3a03cc18b136e592437a3960cf))
* **gemini,chat-v2,notes,providers:** enhance multimodal handling, cache tokens, and batch import cleanup ([b287a23](https://github.com/helixnow/deep-student/commit/b287a237563db711c79e6bda9e7b2933717e6a65))
* **gemini,memory,llm:** add frequency/presence penalties, batch memory write, and provider_scope routing ([958979c](https://github.com/helixnow/deep-student/commit/958979c40c4fee5e82e4c9b5cf5161fbb4df8ba0))


### Bug Fixes

* **ci:** avoid duplicate release creation blocking release-please ([21998cc](https://github.com/helixnow/deep-student/commit/21998cc530f566e3d10f40f9e4097578d0c97194))

## [0.9.31](https://github.com/helixnow/deep-student/compare/v0.9.30...v0.9.31) (2026-03-05)


### Features

* **workflows:** add hotfix workflow for Linux release assets and improve sync reliability ([4b7a71f](https://github.com/helixnow/deep-student/commit/4b7a71fbdc42fec4adc872c86b874713161e6739))


### Bug Fixes

* **chat:** change SessionCard height from fixed to min-height ([cbb156d](https://github.com/helixnow/deep-student/commit/cbb156d89011d51c550762aac35bf142aff725ae))

## [0.9.30](https://github.com/helixnow/deep-student/compare/v0.9.29...v0.9.30) (2026-03-03)


### Features

* add build support for linux ([#41](https://github.com/helixnow/deep-student/issues/41)) ([1d253f2](https://github.com/helixnow/deep-student/commit/1d253f25e78aaf7f3c906943bd30e332059ab4a1))
* **memory:** implement write idempotency and enhance data integrity ([bb18278](https://github.com/helixnow/deep-student/commit/bb1827852b4018fd51de1c3bd78f6368447413d0))
* **vfs:** mark resource as pending after successful unit sync ([77c24f1](https://github.com/helixnow/deep-student/commit/77c24f1218f402e0290b3de5bc8f199d0ebb3454))


### Bug Fixes

* add execute right for build_linux_all.sh ([1d253f2](https://github.com/helixnow/deep-student/commit/1d253f25e78aaf7f3c906943bd30e332059ab4a1))

## [0.9.29](https://github.com/helixnow/deep-student/compare/v0.9.28...v0.9.29) (2026-03-02)


### Features

* **session-management:** introduce session management tools and enhance request handling ([8d26ddb](https://github.com/helixnow/deep-student/commit/8d26ddb4eea67203a6fe18d595bc12b8d6014215))


### Bug Fixes

* **chat-v2:** enforce explicit model resolution for multimodal injection ([be308bf](https://github.com/helixnow/deep-student/commit/be308bf67f0eeb7a3bc14cbf4ef23e7874428434))

## [0.9.28](https://github.com/helixnow/deep-student/compare/v0.9.27...v0.9.28) (2026-03-02)


### Features

* add development scripts for Android environment setup ([ab2953f](https://github.com/helixnow/deep-student/commit/ab2953f4bd35ea1ba657154063ff72bf5dcd4d27))
* **ankiCards:** enhance event handling and error reporting ([6f2642c](https://github.com/helixnow/deep-student/commit/6f2642c428e1dd2512559f1bb14b6faa20a097ba))
* **debug:** implement debug log persistence and filtering options ([fa8f4c9](https://github.com/helixnow/deep-student/commit/fa8f4c9fc99f98ff082890a37beb51ffecbcea5f))
* **exam:** enhance exam XML generation and qbank tools ([fc80777](https://github.com/helixnow/deep-student/commit/fc8077744d99fe66d420552401a69753d6d1b4c6))


### Bug Fixes

* **android-files:** support virtual URI import export flows ([58c4234](https://github.com/helixnow/deep-student/commit/58c4234762a9fa1eec6c7b3f0672069384c1c646))

## [0.9.27](https://github.com/helixnow/deep-student/compare/v0.9.26...v0.9.27) (2026-03-01)


### Features

* enhance Anki card handling with action locks, pagination, and improved error handling ([bf5f2bd](https://github.com/helixnow/deep-student/commit/bf5f2bd189750f8bd971486fce6ea5673323ec21))
* enhance file name handling and import error reporting ([c167b25](https://github.com/helixnow/deep-student/commit/c167b253ee06637c9752ab8437bc30b6d6f9a801))
* implement resource export system with format-specific adapters ([ed6f8f8](https://github.com/helixnow/deep-student/commit/ed6f8f834025b6e5356708948c05556c43c60f1e))
* standardize Tauri v2 parameter naming to camelCase for automatic snake_case mapping ([64f541c](https://github.com/helixnow/deep-student/commit/64f541cbd6d81ce4e03134727678cdcb4362380f))

## [0.9.26](https://github.com/helixnow/deep-student/compare/v0.9.25...v0.9.26) (2026-03-01)


### Features

* enhance bidirectional sync with download-first strategy and improved conflict handling ([4fb78e3](https://github.com/helixnow/deep-student/commit/4fb78e30737575bdbfafab6c24d432b6939754e0))
* enhance file handling with new extraction utilities ([be86d16](https://github.com/helixnow/deep-student/commit/be86d166798455d99cd142808f1c676c4f9cd1a5))
* fix tool call handling and user message deduplication in chat history ([6b38748](https://github.com/helixnow/deep-student/commit/6b3874895b00d1a15dc3d7d87fd0d3fc9f5fe2ff))


### Bug Fixes

* use adapter-transformed request body for LLM request logging ([a93ed02](https://github.com/helixnow/deep-student/commit/a93ed02f9e45c52352035628273196623894cac9))

## [0.9.25](https://github.com/helixnow/deep-student/compare/v0.9.24...v0.9.25) (2026-03-01)


### Features

* add GitHub Actions workflow for rebuilding Android APK ([1285e99](https://github.com/helixnow/deep-student/commit/1285e99643d8f26d61ef2e91d91e11a502e8bd75))
* add image payload parsing and handling utilities ([a16033e](https://github.com/helixnow/deep-student/commit/a16033ef6a27041d11de2a743a5c74f91a013079))
* enhance memory management with new relation and tagging features ([d7dc855](https://github.com/helixnow/deep-student/commit/d7dc8559ee47cdc253a9f71dbe2998808cf774ad))
* enhance model capability registry and update related scripts ([9caea57](https://github.com/helixnow/deep-student/commit/9caea57694f947c92abca1d5bd02cd4eb24c1697))
* enhance sync functionality with merge strategy and timestamp parsing ([274a81e](https://github.com/helixnow/deep-student/commit/274a81ec49a88803d22fd6be6be40d184f813d76))
* implement content search and session tagging system ([cb846b5](https://github.com/helixnow/deep-student/commit/cb846b51741e4fad7ce31d4dfcc0224eba94ff50))
* implement CORS-compliant fetch function for mobile platforms in useAppUpdater ([8206224](https://github.com/helixnow/deep-student/commit/8206224ebae1a6efc9afa0689d7559be7c2cb46a))


### Bug Fixes

* update model capabilities and context token limits ([545d645](https://github.com/helixnow/deep-student/commit/545d64551045f305139be231fa6621cbc4897a5e))

## [0.9.24](https://github.com/helixnow/deep-student/compare/v0.9.23...v0.9.24) (2026-02-27)


### Features

* add ChatAnki integration test plugin for automated testing ([fc20b15](https://github.com/helixnow/deep-student/commit/fc20b15f47590cfe3a21dc813821f16125596b0d))
* add memory audit log functionality and enhance memory management ([24cb17b](https://github.com/helixnow/deep-student/commit/24cb17ba77e7f37b30506cd6bae10457a27e7f16))
* enhance image preview handling and improve NoteContentView layout ([ffe392b](https://github.com/helixnow/deep-student/commit/ffe392bd44da32a28dd9f5725b335dc3bad6492c))
* implement auto-extract frequency settings for memory management ([69a5990](https://github.com/helixnow/deep-student/commit/69a59905f934cad14416c86571ab4fb20f49193f))
* implement automatic migration for GLM-4.1V to GLM-4.6V model ([2d194d9](https://github.com/helixnow/deep-student/commit/2d194d9b35598a1146f418901d02594aa4ff5123))
* introduce release channel management and update README ([4c47987](https://github.com/helixnow/deep-student/commit/4c4798752fa69436f9e16939d015ea2495cc4045))
* update OCR model configurations and enhance engine selection logic ([30097ec](https://github.com/helixnow/deep-student/commit/30097ecdb58b9cb24cb3bc03bf32c6b9f55dea7d))

## [0.9.23](https://github.com/helixnow/deep-student/compare/v0.9.22...v0.9.23) (2026-02-27)


### Bug Fixes

* handle release-please comment failure on locked PRs ([6df5ff8](https://github.com/helixnow/deep-student/commit/6df5ff895eb80e93157e58f82355821ebf29c494))
* resolve TypeScript errors in i18n fallbackLng and IndexStatusView ([00a438a](https://github.com/helixnow/deep-student/commit/00a438a597816de462e51c6e1ab8e58a65e91951))

## [0.9.22](https://github.com/helixnow/deep-student/compare/v0.9.21...v0.9.22) (2026-02-27)


### Features

* add rebuild-release workflow for manual tag rebuilding ([3d28fec](https://github.com/helixnow/deep-student/commit/3d28fec4f6c5fefb794fef3ed2bf2e016a436fb4))

## [0.9.21](https://github.com/helixnow/deep-student/compare/v0.9.20...v0.9.21) (2026-02-26)


### Features

* enhance memory management with auto extraction and category management ([0b5d8fb](https://github.com/helixnow/deep-student/commit/0b5d8fb83158b2811d696852cb6fc7bd07446ace))
* enhance memory management with new settings and export functionality ([2b48b71](https://github.com/helixnow/deep-student/commit/2b48b71e3c33e14ec85fb6f8396d4bdca04dbf18))
* enhance MemoryView with batch selection and editing capabilities ([788147e](https://github.com/helixnow/deep-student/commit/788147e992bdd368b465253308920c7e78eb1402))
* enhance Smart Memory with self-evolving profile and auto-extraction features ([c29005a](https://github.com/helixnow/deep-student/commit/c29005af5e17da3c985bc99e9e510acdddb9d8c5))
* enhance web search tool with dynamic engine injection ([66b5902](https://github.com/helixnow/deep-student/commit/66b590205b828a47f0b449f3b2bd0a608bd6e960))


### Bug Fixes

* correct SQL LIKE pattern escape syntax in note query ([8d96e08](https://github.com/helixnow/deep-student/commit/8d96e08bc5bc5cca947e58f7446db68049a7dc2d))
* increase MCP cache max size for improved performance ([7896e76](https://github.com/helixnow/deep-student/commit/7896e76b09d87ed534041e48d43bd31b08be1cd9))
* prevent action buttons from overlapping session title during edit ([5278d4b](https://github.com/helixnow/deep-student/commit/5278d4beacef6dfa1e63aa85619a490132bf804f))

## [0.9.20](https://github.com/helixnow/deep-student/compare/v0.9.19...v0.9.20) (2026-02-25)


### Features

* add DOCX VLM direct extraction path with streaming and checkpoint recovery ([2ee580f](https://github.com/helixnow/deep-student/commit/2ee580fd8f8465e9a6b867bc505a3e71f38f1fd4))
* add native DOCX import with embedded image support ([304d940](https://github.com/helixnow/deep-student/commit/304d940663577171f8542db8b86e869f2f1274c4))


### Bug Fixes

* improve question import quality and blob path resolution ([aeb5608](https://github.com/helixnow/deep-student/commit/aeb5608115795efbbc99539878d2109ba2f29348))
* update links in README_EN.md for Quick Start and User Guide ([f4611a5](https://github.com/helixnow/deep-student/commit/f4611a5e61463fc88642d30763774b4213e16659))

## [0.9.19](https://github.com/helixnow/deep-student/compare/v0.9.18...v0.9.19) (2026-02-25)


### Bug Fixes

* add fallback logic for empty Anki back field and replace custom scrollbars with CustomScrollArea ([341c9dc](https://github.com/helixnow/deep-student/commit/341c9dc6be4553dff604b9192f8a5bbf92714961))
* prevent duplicate user messages in history and improve IME handling across platforms ([f903bd1](https://github.com/helixnow/deep-student/commit/f903bd18794722fbab566ae932e146cf54428143))
* standardize snippet container heights using Tailwind spacing units ([5fe902d](https://github.com/helixnow/deep-student/commit/5fe902d0e60991ebe4aa1a80b597963220995833))
* update SiliconFlow website URLs in ApisTab and builtin_vendors ([aa2ad0d](https://github.com/helixnow/deep-student/commit/aa2ad0dcb6325b647d0ffbecd08b2047d5ec41c7))

## [0.9.18](https://github.com/helixnow/deep-student/compare/v0.9.17...v0.9.18) (2026-02-25)


### Features

* add data visualization APIs for OCR and text chunk management ([d1b7ae4](https://github.com/helixnow/deep-student/commit/d1b7ae4b74f5deb9d5cf564e88c72197e1164083))
* enhance backup functionality with ImportProgress struct and refactor auto backup logic ([a33f2d9](https://github.com/helixnow/deep-student/commit/a33f2d9a5db03e2a467a834cf064d17f0efe890c))
* implement block and message actions for enhanced chat functionality ([e68df84](https://github.com/helixnow/deep-student/commit/e68df84be6dfc0bf9fface0ebfda9929fff25d0e))


### Bug Fixes

* correct field references and add missing impl block in debug logger ([13bb819](https://github.com/helixnow/deep-student/commit/13bb8194c7d12c9f7a4083c4dacb352a83a54c81))
* prevent duplicate text input during IME composition and sync skill whitelist after load_skills ([05be6b5](https://github.com/helixnow/deep-student/commit/05be6b53a1e392174058a3f9afc6e51256bbe942))


### Performance Improvements

* optimize view switching with memoization and ref-based state tracking ([2dc59c2](https://github.com/helixnow/deep-student/commit/2dc59c2b6a0cb15d2a274579ac91d3108fb787f6))

## [0.9.17](https://github.com/helixnow/deep-student/compare/v0.9.16...v0.9.17) (2026-02-23)


### Features

* enhance SiliconFlowSection with new OCR model and improve backup functionality ([f94fef3](https://github.com/helixnow/deep-student/commit/f94fef323f4fdf536bdc4bc02a7628b839a7d97b))


### Bug Fixes

* enhance error handling and performance optimizations in Chat V2 ([bbaf9ec](https://github.com/helixnow/deep-student/commit/bbaf9ec19b92ef8ce5bc9ee240b6d39b9fd26392))
* gate desktop_dir/picture_dir with #[cfg(desktop)] for Android build ([512768f](https://github.com/helixnow/deep-student/commit/512768f1e1fd7b3d0e9bbf866a471f71ad438b50))
* **gemini:** add thought_signature support for Gemini 3 tool calling and enforce role alternation ([aa82ff0](https://github.com/helixnow/deep-student/commit/aa82ff0d7fdefa14d54f12b7565db3b0d7069a10))
* **gemini:** force v1beta for Gemini 3 models and convert unprotected functionCalls to text ([cd35419](https://github.com/helixnow/deep-student/commit/cd35419616fb2b92996438ae08e302f0ef78ece1))
* **memory:** enforce atomic fact storage and prevent knowledge/content leakage ([dab0c78](https://github.com/helixnow/deep-student/commit/dab0c78383d79b1f4fe3951b6b4b63e54423c48d))

## [0.9.16](https://github.com/helixnow/deep-student/compare/v0.9.15...v0.9.16) (2026-02-22)


### Features

* **chat-v2:** add disable_tool_whitelist option to bypass skill whitelist restrictions ([830d1eb](https://github.com/helixnow/deep-student/commit/830d1eb815a8e8bd1386064d06aa97a3e6c04d04))
* 题目集导入断点续导（checkpoint resume） ([6ef1333](https://github.com/helixnow/deep-student/commit/6ef1333e92f6977c6f072223e66ae0a7227a4045))


### Bug Fixes

* address verified P0/P1 issues from code audit ([0dca38e](https://github.com/helixnow/deep-student/commit/0dca38e5761c670a4f5d6681f0a50dadb283239a))
* **chat-v2:** ensure active skills content is always passed to backend for synthetic load_skills injection ([0f791c0](https://github.com/helixnow/deep-student/commit/0f791c074fb7fdaf87c7e39a50747df2531beafc))
* **mcp:** audit compliance fixes - timeout alignment, connection state tracking, and DRY refactor ([4fbb093](https://github.com/helixnow/deep-student/commit/4fbb093ef85ea0fdd0e19e43bc44d9316dac0147))
* **mcp:** sanitize tool names for OpenAI API compatibility and improve memory retrieval ranking ([2bf3d9f](https://github.com/helixnow/deep-student/commit/2bf3d9fd34fed8d569dc0b666e7244c5c1e186cb))
* **web-search:** remove engine/force_engine from schema and add silent fallback for unconfigured engines ([e136ef8](https://github.com/helixnow/deep-student/commit/e136ef8206c9bcc3c933cd0a8c635d70f2cfc407))

## [0.9.15](https://github.com/helixnow/deep-student/compare/v0.9.14...v0.9.15) (2026-02-21)


### Features

* **mindmap:** add rich text formatting toolbar and emoji picker, improve node styling and export ([36981fb](https://github.com/helixnow/deep-student/commit/36981fbe1ee5578355128f7d26c69ae106c5cfbf))


### Bug Fixes

* **essay-grading:** replace description Input with textarea for multi-line mode descriptions ([881bd5e](https://github.com/helixnow/deep-student/commit/881bd5e97c72c4cc82b85e1e2ea302d4b70b00fe))

## [0.9.14](https://github.com/helixnow/deep-student/compare/v0.9.13...v0.9.14) (2026-02-20)


### Features

* **chat-v2:** add session branching and group pinned resources support ([82f359c](https://github.com/helixnow/deep-student/commit/82f359cb9ad3ca77cca01a2082f37b5c4ff747ce))
* **chat-v2:** use dedicated chat_title_model for summary generation with fallback chain ([eb5e14d](https://github.com/helixnow/deep-student/commit/eb5e14d425a49606373de786e8dc6c27fded302b))
* **cloud-sync:** add real-time upload/download progress events and workspace database backup support ([8a2b496](https://github.com/helixnow/deep-student/commit/8a2b496ab3b6c84a59327fce896c721d9545c8c4))
* **essay-grading:** refine grading mode rubrics and implement progressive hedging for OCR fallback ([40f2664](https://github.com/helixnow/deep-student/commit/40f2664c44f3be55fab52c54f6ca69737c8c13fb))
* **ocr:** add FreeOCR fallback chain with circuit breaker and streamline grading mode prompts ([6777d50](https://github.com/helixnow/deep-student/commit/6777d501aa9820d599701faea26114e70608209f))
* **settings:** add vendor model batch import and refactor essay grading settings panel ([b282fdb](https://github.com/helixnow/deep-student/commit/b282fdb451db75717f83e6f4614aa20ab8df310c))
* **sync:** add workspace database and VFS blob file-level cloud sync support ([bccce85](https://github.com/helixnow/deep-student/commit/bccce85b2cee4c4a8147364874ee549c05e4ec94))
* **vfs:** filter deleted/inactive resources in index status queries and add question filtering in exam uploader ([1665d05](https://github.com/helixnow/deep-student/commit/1665d0512a5d2fa0bc93c0fb71142cae3adbac08))


### Bug Fixes

* **android:** replace navigator.clipboard with tauri-plugin-clipboard-manager ([d410dc2](https://github.com/helixnow/deep-student/commit/d410dc2eb08b5f3b1cfff06cdec329f3688ade5d))
* **chat-v2:** fix continue message error handling and builtin model badge display logic ([2b20f3a](https://github.com/helixnow/deep-student/commit/2b20f3a705e014a7ba9422b7ea1c1ec4b1827225))
* **chat-v2:** reorder session branching DB writes to satisfy FK constraints and refactor resource picker UI ([185137c](https://github.com/helixnow/deep-student/commit/185137c1bf9177e44bc3fb88acc588c00705a4ed))
* merge duplicate clipboardUtils import in useMindMapClipboard ([fd71294](https://github.com/helixnow/deep-student/commit/fd712942470c2ece3ab6a877d0e8f0ea68df4764))

## [0.9.13](https://github.com/helixnow/deep-student/compare/v0.9.12...v0.9.13) (2026-02-18)


### Features

* add multi-tab support with LRU eviction, fix cross-tab event pollution, and enhance LaTeX rendering ([8af002c](https://github.com/helixnow/deep-student/commit/8af002cc7d29e53092f70d1441be006597cea394))
* enhance tool handling, sleep wake logic, and crypto key backup/restore ([a477bca](https://github.com/helixnow/deep-student/commit/a477bca302fb8d487a5e43a64b56aaad9450651f))
* **indexing:** 一键索引自动对预处理未完成的教材/PDF文件执行OCR ([83560f7](https://github.com/helixnow/deep-student/commit/83560f7968b7957fe70be62e955a48f4565cfdcc))


### Performance Improvements

* **vfs:** optimize index status query with CTE aggregation and add performance indexes ([07c6e5e](https://github.com/helixnow/deep-student/commit/07c6e5ea479bf9b0f888642572693755d4e17530))

## [0.9.12](https://github.com/helixnow/deep-student/compare/v0.9.11...v0.9.12) (2026-02-18)


### Features

* add backup cancellation support and fix attachment base64 detection ([18bbc22](https://github.com/helixnow/deep-student/commit/18bbc223f3f06e6c447f6b6cd2e5de7a00e8932d))

## [0.9.11](https://github.com/helixnow/deep-student/compare/v0.9.10...v0.9.11) (2026-02-17)


### Features

* enhance progress tracking for backup/restore/import operations with detailed error reporting ([9fb24a4](https://github.com/helixnow/deep-student/commit/9fb24a41147ebdb2ee38819f0821ac8e76894bd6))

## [0.9.10](https://github.com/helixnow/deep-student/compare/v0.9.9...v0.9.10) (2026-02-17)


### Features

* mobile dual download links (R2 mirror + GitHub) ([c9c8f6d](https://github.com/helixnow/deep-student/commit/c9c8f6dc583cf01b652a6b0c5378dcbdc0e41125))
* prioritize R2 mirror for auto-update source ([7e479c8](https://github.com/helixnow/deep-student/commit/7e479c8955bbc820afbfa424472a81cd48138185))
* source image crop, search snippets, remove question_parsing_model ([d41f6c0](https://github.com/helixnow/deep-student/commit/d41f6c09ff6c503194264f6da3048397a4e9877f))


### Bug Fixes

* add --remote flag to wrangler r2 commands ([f7068ef](https://github.com/helixnow/deep-student/commit/f7068ef2911443a4325d98a1c7798cdbfd7b8cc2))
* **backup:** configure git user for annotated snapshot tags in bare repo ([6bc2fb4](https://github.com/helixnow/deep-student/commit/6bc2fb4c6d7735623a2e0deaaf7c023b19b7c09d))
* **ci:** prevent dependabot major bumps + precise semver extraction ([b6396bc](https://github.com/helixnow/deep-student/commit/b6396bc73d2a9c7a9d5d61d785d7934e34565bb4))
* critical review fixes for R2 upload in release workflow ([5f616dc](https://github.com/helixnow/deep-student/commit/5f616dc69929005ca8d4a856f64347826501ac1d))
* **release:** disable component-prefixed tags + robust version extraction ([f4bafa4](https://github.com/helixnow/deep-student/commit/f4bafa4822e19881f6c12167d7aa5df60b2cb0d6))
* switch to rclone for R2 upload (native Cloudflare provider) ([d3aebda](https://github.com/helixnow/deep-student/commit/d3aebdab15fc33108c54e1d0ec46e50fdcfb59b6))
* switch to wrangler CLI for R2 upload (bypass S3 TLS issue) ([0272c39](https://github.com/helixnow/deep-student/commit/0272c3963b7d012b3e8500b88f2b8271c8cb3961))
* **updater:** robust version extraction from tag_name for Android ([4be6c1f](https://github.com/helixnow/deep-student/commit/4be6c1fde614fb44b0d9e3a2bad332e86dfacd80))
* use GitHub API for R2 version cleanup (wrangler has no list command) ([41cedb4](https://github.com/helixnow/deep-student/commit/41cedb4c0d68d82e8dd425308194d6c78c8703f1))
* use path-style addressing for R2 S3 compatibility ([c26433d](https://github.com/helixnow/deep-student/commit/c26433db37c04ae5ac7f1e13c542a9c3d5d7dfe1))


### Performance Improvements

* add cache-control headers and proper content-types for R2 uploads ([333d96d](https://github.com/helixnow/deep-student/commit/333d96dd73b903ead76a07182a43c94bda277617))

## [0.9.9](https://github.com/helixnow/deep-student/compare/deep-student-v0.9.8...deep-student-v0.9.9) (2026-02-17)


### Bug Fixes

* **android:** disable ppt-rs default features to avoid openssl-sys ([6a3acc7](https://github.com/helixnow/deep-student/commit/6a3acc7c278c3a839849e6d4b46a24895067c1ca))

## [0.9.8](https://github.com/helixnow/deep-student/compare/deep-student-v0.9.7...deep-student-v0.9.8) (2026-02-17)


### Features

* add academic search tool with arXiv + OpenAlex integration ([1ae5c24](https://github.com/helixnow/deep-student/commit/1ae5c24534afe33addc0980801bde18869b79e4a))
* add Android build to release workflow + bump VERSION_CODE_BASE to 13000 ([54c0d22](https://github.com/helixnow/deep-student/commit/54c0d22407b305c32df90a9848225637f4c9fe4f))
* add attachment pipeline automated test plugin ([371e5c5](https://github.com/helixnow/deep-student/commit/371e5c5a6f830475cffb70f65480c2c17153495b))
* add database maintenance mode + fix Windows file lock (OS error 32) during restore ([7023510](https://github.com/helixnow/deep-student/commit/7023510b76afcb23149ba0271e9c020c102c9608))
* add orphan OCR engine cleanup + improve file save UX + fix test engine selection ([b080582](https://github.com/helixnow/deep-student/commit/b08058212f4cb360ba87bf96dd41721eb772fc37))
* add paper save + citation formatting tools with VFS integration ([176aae2](https://github.com/helixnow/deep-student/commit/176aae2b49fd03b3d6ed0a4c636fa08e644e5aaf))
* cross-platform pdfium fixes + system OCR adapters + platform-specific resource bundling ([ea87e01](https://github.com/helixnow/deep-student/commit/ea87e015a84e1da8c5ed32b9679de0d7298f9db1))
* improve mobile UI layout + migrate template buttons to NotionButton ([afd62b4](https://github.com/helixnow/deep-student/commit/afd62b4bb278f8790ff9918e0080e6d8cc36939f))
* integrate release-please for automated release management ([69db429](https://github.com/helixnow/deep-student/commit/69db42973bf69849e730f25a61d80129a3b767ce))
* **tools:** add DOCX document read/write tool executor + Excel/PowerPoint dependencies ([2a7546a](https://github.com/helixnow/deep-student/commit/2a7546a942b55d8bbf163f6e22ea9239d1baf988))
* **tools:** add PPTX/XLSX tool executors with full read/write capabilities ([d3f6bc5](https://github.com/helixnow/deep-student/commit/d3f6bc52d5899a7def675f16adb815bd08536421))


### Bug Fixes

* add empty string clearing for group fields + validate group existence + cleanup vector indices on delete/purge ([754da80](https://github.com/helixnow/deep-student/commit/754da807a666d8cf4fe80a901638aa2f3c66999d))
* add generate-version.mjs to all platform builds + update committed version ([2f0cfec](https://github.com/helixnow/deep-student/commit/2f0cfec870d15e29f1ef2ec4082b13ba2109ddc1))
* add process:default capability + harden semver comparison ([78bff18](https://github.com/helixnow/deep-student/commit/78bff1854e0a2c4b1fb8d3373b986013e2885b09))
* add protoc install for macOS (brew) and Windows (choco) in release builds ([69e67f0](https://github.com/helixnow/deep-student/commit/69e67f0113f99ba9410de90d1ef32966d128b085))
* bump VERSION_CODE_BASE to 10000 + Node 22 + memory fix for release builds ([8143f02](https://github.com/helixnow/deep-student/commit/8143f02c424ddf2c59973fea27c97e15f8837662))
* copy custom Android icons after tauri android init in CI ([f69ab56](https://github.com/helixnow/deep-student/commit/f69ab56cb6a45d9d15247c23ea7a13c4725a52a2))
* **deps:** migrate json_validator to jsonschema 0.42 API ([a044d95](https://github.com/helixnow/deep-student/commit/a044d95869a2b3f714693a67b18792139101aed4))
* downgrade pdfium to 7350 + add diagnostic command + repair stale PDF cache + harden ready_modes validation ([92a317c](https://github.com/helixnow/deep-student/commit/92a317c8d6c6c82019d596a38ee3d6df0fa974c2))
* enable createUpdaterArtifacts for Tauri v2 updater ([6ca2e5c](https://github.com/helixnow/deep-student/commit/6ca2e5c0410fddc07f91e09d7c581113b845cd52))
* harden migration backup validation + auto-backfill PDF processing status + improve test plugin model handling ([1e23842](https://github.com/helixnow/deep-student/commit/1e238422f6def557b8b1b498a156eed8b51a3ed4))
* improve tool call argument parsing + add paper save fallback handling + add purge safety checks ([bf94e37](https://github.com/helixnow/deep-student/commit/bf94e3753fbed6c48450424e286d3da629fde6d2))
* improve tool schema parameter formats to reduce LLM confusion ([2b24b1e](https://github.com/helixnow/deep-student/commit/2b24b1ea7248ac25849f3b3db233b0475059957d))
* mobile updater uses semver comparison instead of string inequality ([612c250](https://github.com/helixnow/deep-student/commit/612c25033d623d1eb4a8aef83fe306ee061491d5))
* platform-aware auto-updater for all platforms ([29651ad](https://github.com/helixnow/deep-student/commit/29651ad3c1d58232d50b452fbb6d0e4740e04d7c))
* release workflow critical fixes ([0c3b404](https://github.com/helixnow/deep-student/commit/0c3b404b599af69b5b4cee7ed7a1b1e4c22ae650))
* remove custom OCR prompts + harden attachment test plugin ([7c3e43d](https://github.com/helixnow/deep-student/commit/7c3e43de723620d35675e75b39ab10d03b709727))
* remove default Tauri drawables + restrict mobile.json to mobile platforms ([ca43bb3](https://github.com/helixnow/deep-student/commit/ca43bb3aa1560e1fc95424cd2d06c93a0ff12993))
* remove Gemini OpenAI compat mode special handling + add OCR diagnostic logging ([5063706](https://github.com/helixnow/deep-student/commit/50637067311e65a5ea173a4e57ddae0db2e3ca0b))
* rename macOS .app.tar.gz with arch suffix to prevent overwrite ([a7936cb](https://github.com/helixnow/deep-student/commit/a7936cb77bb6807481371f20be0f7d05a238ac04))
* resolve TypeScript type errors in attachment audit logging ([499a41b](https://github.com/helixnow/deep-student/commit/499a41b5af3d8a34769a6b77cd9db37c5f22b1db))
* **restore:** 恢复备份写入非活跃插槽，避免 Windows OS error 32 ([af6c11f](https://github.com/helixnow/deep-student/commit/af6c11f89a51f47d88035172f83bf0a9f63f44e5))
* restrict desktop capabilities to desktop platforms + misc improvements ([6772c17](https://github.com/helixnow/deep-student/commit/6772c17932d553c8908acc562a8d2e81eaeac817))
* show 'already up to date' feedback after manual update check ([e7b27fe](https://github.com/helixnow/deep-student/commit/e7b27fe2ccb6c44a3f3f6796f761895ec45e9e98))
* use arduino/setup-protoc, fail-fast false, remove redundant frontend build ([1ddf626](https://github.com/helixnow/deep-student/commit/1ddf6268e583e8a9bbda4afd26458ed28d335f34))

## [Unreleased] | 未发布

---

## [0.9.7] - 2026-02-16

### Fixed | 修复
- 修复 v0.9.6 发布构建产物版本号错误的问题（版本文件未正确 bump）

### Changed | 变更
- 规范 release 流程：版本 bump 必须通过 release-please PR 合并，禁止手动 tag

---

## [0.9.6] - 2026-02-15

### Added | 新增
- 数据库维护模式，支持备份恢复期间自动切换
- 英文 README 及双语导航链接
- 翻译工作台功能及截图文档
- Anki 模板截图文档更新 + 最新 LLM 模型（GLM-5, Seed 2.0, M2.5, GPT-5.2 Pro）

### Fixed | 修复
- 修复恢复备份写入非活跃插槽，避免 Windows OS error 32 文件锁问题

### Changed | 变更
- CI 移除 cargo fmt 检查 + 按钮迁移到 NotionButton 组件

---

## [0.9.5] - 2026-02-13

### Added | 新增
- 安全政策文档 (`SECURITY.md`)
- 环境变量示例 (`.env.example`)
- Playwright E2E 测试配置
- CI/CD 流水线配置 (`.github/workflows/ci.yml`)
- 第三方许可证清单 (`THIRD_PARTY_LICENSES.md`)

### Changed | 变更
- 移除贡献者许可协议文档（待议）

### Fixed | 修复
- 修复 `test:e2e` 脚本缺失问题

---

## [0.9.1] - 2026-02-12

### Added | 新增
- ChatAnki 端到端制卡闭环（替代原 CardForge 独立制卡流程）
- Skills 渐进披露架构：工具按需注入，显著减少上下文占用
- 内置技能：`tutor-mode`、`chatanki`、`literature-review`、`research-mode`
- 内置工具组：`knowledge-retrieval`、`canvas-note`、`vfs-memory`、`todo-tools` 等 11 个
- 数据治理面板：集中化备份、同步、审计、迁移管理
- 云同步功能：WebDAV 和 S3 兼容存储支持
- 双槽位数据空间 A/B 切换机制
- 外部搜索引擎：新增智谱 AI 搜索、博查 AI 搜索
- MCP 预置服务器：Context7 文档检索
- 命令面板：支持收藏、自定义快捷键、拼音搜索
- 3D 卡片预览与多风格内置模板（11 种设计风格）
- 多模态精排模型支持
- 子代理工作器（subagent-worker）技能

### Changed | 变更
- 模型分配简化：移除第一模型、深度研究模型、总结生成模型，统一使用对话模型
- 备份设置迁移到数据治理面板
- 底部导航栏改为 5 个直接 Tab（移除"更多"折叠菜单）
- MCP 预置服务器精简为仅 Context7

### Fixed | 修复
- 修复移动端底部导航栏布局
- 修复多个命令面板快捷键冲突

---

## [0.9.0] - 2026-01-31

### Added | 新增
- Chat V2 架构：支持多轮对话、消息编辑、流式响应
- MCP (Model Context Protocol) 工具生态集成
- VFS 统一资源存储系统
- 双槽位数据空间与迁移机制
- AES-256-GCM 安全存储
- 国际化支持 (i18n)
- 深色/浅色主题切换
- PDF/Word/PPT 文档预览
- 知识图谱可视化
- 错题本与 Anki 导出

### Changed | 变更
- 升级 Tauri 至 v2.x
- 重构前端状态管理（Zustand）
- 优化移动端 UI 适配

### Fixed | 修复
- 修复 Android WebView 兼容性问题
- 修复大文件上传内存溢出
- 修复会话切换时的状态泄漏

---

## [0.8.9] - 2024-11-30

### Added | 新增
- 初始公开版本
- 基础聊天功能
- 多模型供应商支持
- 本地优先数据存储

---

[Unreleased]: https://github.com/helixnow/deep-student/compare/v0.9.17...HEAD
[0.9.7]: https://github.com/helixnow/deep-student/compare/v0.9.6...v0.9.7
[0.9.6]: https://github.com/helixnow/deep-student/compare/v0.9.5...v0.9.6
[0.9.5]: https://github.com/helixnow/deep-student/compare/v0.9.1...v0.9.5
[0.9.1]: https://github.com/helixnow/deep-student/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/helixnow/deep-student/compare/v0.8.9...v0.9.0
[0.8.9]: https://github.com/helixnow/deep-student/releases/tag/v0.8.9
