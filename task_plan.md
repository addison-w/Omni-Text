# Task Plan: Omni Text — macOS Hotkey Rewrite App

## Goal
Build a macOS menu bar app ("Omni Text") that lets users select text anywhere, press a custom hotkey, send that text to an OpenAI-compatible LLM API, then replace the selected text with the model's output — with a Liquid Glass UI aligned to macOS Tahoe design.

## Current Phase
Phase 1 (complete) → Ready for Phase 2

## Phases

### Phase 1: Requirements & Research
- [x] Analyze PRD and extract requirements
- [x] Research MenuBarExtra API, global hotkeys, Accessibility APIs
- [x] Research Keychain, SwiftData, Liquid Glass
- [x] Generate UI/UX design system
- [x] Document findings in findings.md
- **Status:** complete

### Phase 2: Project Setup & Scaffold
- [ ] Initialize Xcode project (macOS App, SwiftUI lifecycle)
- [ ] Configure Info.plist: LSUIElement = YES
- [ ] Set up entitlements (non-sandboxed for Accessibility)
- [ ] Add Swift Package dependencies:
  - KeyboardShortcuts (sindresorhus)
  - KeychainAccess (kishikawakatsumi)
- [ ] Create folder structure:
  ```
  OmniText/
  ├── App/
  │   └── OmniTextApp.swift
  ├── Models/
  │   ├── RewriteAction.swift
  │   ├── HistoryEntry.swift
  │   ├── ProviderConfig.swift
  │   └── AppState.swift
  ├── Services/
  │   ├── TextInteractionService.swift
  │   ├── LLMProviderService.swift
  │   ├── HotkeyManager.swift
  │   ├── KeychainService.swift
  │   └── HistoryService.swift
  ├── Views/
  │   ├── MenuBar/
  │   │   ├── MenuBarView.swift
  │   │   └── StatusIconView.swift
  │   ├── Settings/
  │   │   ├── SettingsView.swift
  │   │   ├── PromptsHotkeysTab.swift
  │   │   ├── APIProvidersTab.swift
  │   │   ├── HistoryTab.swift
  │   │   ├── PrivacySafetyTab.swift
  │   │   └── AdvancedTab.swift
  │   ├── Onboarding/
  │   │   └── OnboardingView.swift
  │   ├── Components/
  │   │   ├── GlassCard.swift
  │   │   ├── GlassButton.swift
  │   │   ├── ToastView.swift
  │   │   └── HotkeyRecorderView.swift
  │   └── Shared/
  │       └── LiquidGlassModifiers.swift
  ├── Utilities/
  │   ├── AccessibilityHelper.swift
  │   └── Constants.swift
  ├── Assets.xcassets/
  └── Info.plist
  ```
- [ ] Create basic App entry point with MenuBarExtra
- [ ] Verify builds and runs as menu bar app
- **Status:** pending

### Phase 3: Core Infrastructure — Models & State
- [ ] Define SwiftData models:
  - `HistoryEntry`: id, timestamp, appName, actionName, originalText, resultText, providerName, modelName, duration, tokenUsage
  - `RewriteAction`: id, name, hotkeyCombination, systemPrompt, userPromptTemplate, outputRules, modelOverride, sortOrder, isDefault
  - `ProviderConfig`: id, name, baseURL, apiKey (ref to Keychain), modelName, timeout, customHeaders, isActive
- [ ] Create `AppState` (ObservableObject):
  - isEnabled, isProcessing, currentError, privacyMode
  - activeProvider, actions array
  - permission states (accessibility, input monitoring)
- [ ] Set up SwiftData ModelContainer in App
- [ ] Create `KeychainService` wrapper using KeychainAccess
- [ ] Seed default actions (Paraphrase, Grammar Fix) on first launch
- **Status:** pending

### Phase 4: Text Interaction Layer
- [ ] Create `AccessibilityHelper`:
  - Check/request Accessibility permission
  - Get focused element
  - Read selected text from any app
  - Replace selected text
  - Detect secure fields (password inputs)
  - Detect empty selection
- [ ] Create `ClipboardFallback`:
  - Save current pasteboard
  - Simulate Cmd+C, read clipboard
  - Write replacement, simulate Cmd+V
  - Restore original pasteboard (with delay)
  - Respect TransientType / ConcealedType markers
- [ ] Create `TextInteractionService`:
  - Try Accessibility first → fall back to clipboard
  - Return clear error if both fail
  - Log which method was used
- [ ] Test in: Safari, Notes, Slack, VS Code, Chrome
- **Status:** pending

### Phase 5: LLM Provider Layer
- [ ] Create `LLMProviderService`:
  - Build OpenAI-compatible Chat Completions request
  - Support: base URL, API key, model name, timeout, custom headers
  - Send system prompt + user prompt with selected text
  - Parse response: extract plain text content
  - Handle errors: timeout, 4xx, 5xx, network, malformed response
  - Optional single retry on 5xx/timeout
- [ ] Create `ResponseNormalizer`:
  - Strip quotes, headings, markdown artifacts, "Here's your rewrite:" prefixes
  - Detect empty/identical output → return nil
  - Preserve basic formatting if action's outputRules say so
- [ ] Create connection test function:
  - Send small test prompt
  - Return: success/failure, latency, model info, error details
- [ ] Wire up: hotkey press → get text → call LLM → replace text
- **Status:** pending

### Phase 6: Hotkey System
- [ ] Create `HotkeyManager` using KeyboardShortcuts:
  - Register all active action hotkeys on app launch
  - Re-register when actions are added/edited/deleted
  - Handle hotkey press → trigger `TextInteractionService` + `LLMProviderService` pipeline
- [ ] Hotkey capture UX:
  - Use `KeyboardShortcuts.Recorder` in settings
  - Detect conflicts (duplicate within app + known system shortcuts)
  - Show warning for conflicts, allow override
- [ ] Wire hotkey press to full pipeline:
  1. Check isEnabled
  2. Check selection not empty / not secure field
  3. Set isProcessing = true
  4. Get selected text
  5. Call LLM with action's prompt
  6. Replace text
  7. Set isProcessing = false
  8. Show success toast
  9. Save to history (if not private mode)
- **Status:** pending

### Phase 7: UI — Liquid Glass Components
- [ ] Create `LiquidGlassModifiers.swift`:
  - `.glassBackground()` — applies .glassEffect() on macOS 26, falls back to .ultraThinMaterial
  - `.glassCard()` — rounded rect with glass + border + shadow
  - `.glassButton()` — capsule glass button style
- [ ] Create `GlassCard` component
- [ ] Create `GlassButton` / `GlassButtonStyle`
- [ ] Create `ToastView`:
  - Floating notification near top-right
  - Auto-dismiss after 2s
  - Success (checkmark), Error (x), Info states
  - Glass background
- [ ] Create `HotkeyRecorderView`:
  - Wraps `KeyboardShortcuts.Recorder`
  - Shows current hotkey or "Record Shortcut..."
  - Conflict indicator
- **Status:** pending

### Phase 8: UI — Menu Bar & Settings Window
- [ ] **MenuBarView** (popover via .window style):
  - App name + status indicator
  - Toggle "Enabled" switch
  - Toggle "Private Mode" switch
  - "Test Connection" button
  - Quick access to recent actions
  - "Settings..." button → opens settings window
  - "Quit" button
  - Glass background
- [ ] **SettingsView** (tabbed window):
  - Tab bar: Prompts & Hotkeys | API Provider | History | Privacy & Safety | Advanced
  - Glass material background
- [ ] **PromptsHotkeysTab**:
  - List of actions (name, hotkey badge, edit/delete)
  - Add action button
  - Edit sheet: name, hotkey recorder, system prompt, user template, output rules
  - Reorder (drag or up/down)
- [ ] **APIProvidersTab**:
  - Provider name, base URL, API key (secure field), model name
  - Timeout slider
  - Optional headers (key-value list)
  - Test Connection button with status indicator
- [ ] **HistoryTab**:
  - List: timestamp, action name, original preview, result preview
  - Search bar
  - Filter by action/date
  - Detail view: full original + result, copy buttons
  - Delete single / clear all
- [ ] **PrivacySafetyTab**:
  - Private mode toggle (don't store history)
  - "Your text is sent to: [provider name]" disclosure
  - Clear all history button
  - Per-app exclusion list (v2 — placeholder)
- [ ] **AdvancedTab**:
  - Launch at Login toggle (SMAppService)
  - Clipboard fallback mode toggle
  - Retry on failure toggle
  - Reset to defaults
  - App version info
- **Status:** pending

### Phase 9: Onboarding Flow
- [ ] **OnboardingView** (shown on first launch):
  - Step 1: Welcome + what the app does
  - Step 2: Set up provider (base URL, model, API key)
  - Step 3: Test connection
  - Step 4: Grant Accessibility permission (with guide)
  - Step 5: Try a sample rewrite on a test field
  - Progress indicator (step dots)
  - Skip option
- [ ] Track onboarding completion in UserDefaults
- [ ] Show "Not Ready" state if onboarding incomplete
- **Status:** pending

### Phase 10: Status, Errors & Polish
- [ ] Menu bar icon states:
  - Ready: SF Symbol `text.bubble.fill` (template)
  - Processing: animated SF Symbol or custom spinner
  - Not Ready: `exclamationmark.triangle.fill` (template)
- [ ] Error handling:
  - Toast for user-facing errors
  - Menu bar tooltip with last error
  - Detailed error view in settings (raw response)
- [ ] Cancel in-flight request (set flag, ignore late response)
- [ ] Undo guidance: show "Press ⌘Z to undo" in success toast
- [ ] Empty selection guard
- [ ] Secure field detection guard
- [ ] Performance: async/await throughout, never block main thread
- [ ] Verify < 1.5s median latency on fast endpoints
- **Status:** pending

### Phase 11: Testing & Verification
- [ ] Unit tests:
  - ResponseNormalizer
  - LLMProviderService (mock network)
  - TextInteractionService (mock AX)
  - HotkeyManager registration
- [ ] Integration tests:
  - Full pipeline: hotkey → capture → LLM → replace
  - History recording
  - Keychain storage/retrieval
- [ ] Manual testing matrix:
  - Safari, Chrome, Notes, Slack, Mail, VS Code
  - Test with OpenAI API
  - Test with local LM Studio / Ollama
  - Test empty selection, secure fields
  - Test cancel mid-request
  - Test provider errors (bad key, timeout, unreachable)
- [ ] UI review:
  - Glass effects render correctly
  - Light + dark mode
  - All tabs functional
  - Onboarding flow complete
- **Status:** pending

### Phase 12: Delivery
- [ ] Review all source files
- [ ] Verify clean build (no warnings)
- [ ] Initialize git repository
- [ ] Create README with setup instructions
- [ ] Deliver to user
- **Status:** pending

## Key Questions
1. ~~What SwiftUI APIs to use for menu bar?~~ → MenuBarExtra with .window style
2. ~~How to capture global hotkeys?~~ → KeyboardShortcuts library
3. ~~How to read/replace selected text?~~ → Accessibility API primary, clipboard fallback
4. ~~SwiftData or CoreData?~~ → SwiftData (macOS 15+)
5. ~~How to implement Liquid Glass?~~ → .glassEffect() on macOS 26, .ultraThinMaterial fallback
6. Should we target macOS 15 minimum or macOS 26 only? → **TBD — recommend macOS 15+ with Tahoe enhancements**
7. Distribution: Developer ID signing or unsigned for personal use? → **TBD**

## Decisions Made
| Decision | Rationale |
|----------|-----------|
| MenuBarExtra (.window style) | Native SwiftUI, supports complex popover content |
| KeyboardShortcuts library | Best SwiftUI integration, recorder UI, conflict detection |
| Accessibility API + clipboard fallback | Most reliable text capture strategy |
| KeychainAccess library | Simple API, secure defaults, SwiftUI-friendly |
| SwiftData | Modern, type-safe, perfect SwiftUI integration |
| Non-sandboxed | Required for Accessibility API |
| SF Symbols for icons | Native, template rendering, scales properly |
| .glassEffect() with fallback | Native Liquid Glass on Tahoe, graceful degradation |
| Async/await throughout | Modern Swift concurrency, non-blocking |
| macOS 15+ minimum target | Broad compatibility, SwiftData available |

## Errors Encountered
| Error | Attempt | Resolution |
|-------|---------|------------|
| (none yet) | - | - |

## Notes
- macOS 26 has a known issue with `openSettings()` in MenuBarExtra — use hidden WindowGroup workaround
- Accessibility API requires non-sandboxed app — cannot distribute via Mac App Store
- KeyboardShortcuts stores shortcuts in UserDefaults — persist across launches
- NSPasteboard special types (TransientType, ConcealedType) must be respected
- SwiftData VersionedSchema should be set up from day one for future migrations
