---
name: aichat
status: in-progress
created: 2025-10-06T13:10:22Z
progress: 13%
prd: .claude/prds/aichat.md
github: https://github.com/johnproblems/aichat/issues/1
---

# Epic: AIChat Web GUI Transformation

## Overview

Transform AIChat from a CLI-only LLM tool into a comprehensive web-based platform with Matrix-themed interface, API key marketplace, document processing, and cloud integrations. The foundation (Tasks 1-2) is complete with Matrix theme and authentication system operational. This epic focuses on the remaining 7 core feature implementations (Tasks 3-9), broken into 35 detailed subtasks.

**Current Status:** 13% complete (2 of 9 core tasks done)
- ✅ Task 1: Matrix Console Theme Foundation (COMPLETE)
- ✅ Task 2: Database Schema and User Management (COMPLETE)
- 🔄 Tasks 3-9: In Progress

## Architecture Decisions

### Leverage Existing Infrastructure
- **Rust Backend:** Build on existing `src/serve.rs` Hyper-based server
- **Multi-Provider System:** Extend `src/client/` for marketplace routing
- **RAG System:** Utilize `src/rag/` for document processing
- **Session Management:** Enhance `src/config/session.rs` for web users
- **Crypto Utilities:** Use existing `utils/crypto.rs` for API key encryption

### Key Technology Choices
- **Database:** Supabase/PostgreSQL (already integrated)
- **Payments:** Stripe API for marketplace transactions
- **Terminal:** xterm.js for web-based CLI emulation
- **Cloud Storage:** Google Drive + OneDrive APIs
- **Web Search:** DuckDuckGo (free) + Google Search (premium)
- **Frontend:** Matrix-themed HTML/CSS/JS with WebSockets
- **Analytics:** Custom Rust backend with PostgreSQL aggregation

### Design Patterns
- **Middleware Pattern:** JWT authentication, rate limiting, error handling
- **Repository Pattern:** Database operations abstraction
- **Strategy Pattern:** Multiple payment/cloud/search providers
- **Observer Pattern:** Real-time updates via WebSockets
- **Circuit Breaker:** API failover and capacity management

## Technical Approach

### Frontend Components
- **Marketplace UI:** Listing browser, purchase flow, provider dashboard
- **Terminal Component:** xterm.js with WebSocket communication
- **Multi-Provider Chat:** Side-by-side response comparison UI
- **Document Browser:** Cloud file picker with upload progress
- **Analytics Dashboard:** Usage stats, cost tracking, reports
- **Session Manager:** Conversation history with search
- **Search Interface:** Web search integration with citations

### Backend Services
- **Marketplace Service:**
  - API key CRUD operations
  - Dynamic pricing engine
  - Request routing with capacity tracking
  - Failover mechanisms

- **Billing Service:**
  - Stripe integration
  - Balance management
  - Transaction tracking
  - Payout processing

- **Terminal Service:**
  - WebSocket endpoint for CLI commands
  - Session synchronization
  - Command routing to aichat backend

- **Document Service:**
  - Cloud API integration (GDrive, OneDrive)
  - File processing pipeline
  - RAG integration

- **Analytics Service:**
  - Data collection middleware
  - Aggregation queries
  - Report generation

- **Session Service:**
  - Web session management
  - Context summarization
  - Collaboration features

- **Search Service:**
  - Multi-provider search aggregation
  - Citation extraction
  - Context integration

### Infrastructure
- **Deployment:** Docker containers with existing Rust build
- **Database:** PostgreSQL via Supabase (schema already migrated)
- **Authentication:** JWT middleware (already implemented)
- **WebSockets:** Hyper WebSocket upgrade for terminal/real-time
- **Monitoring:** Logging with tracing crate, metrics collection
- **Scaling:** Horizontal scaling for marketplace routing

## Implementation Strategy

### Phase 1: Marketplace & Payments (High Priority)
**Tasks 3.x & 8.x** - Enable monetization core
- Weeks 1-4: Marketplace backend and API endpoints
- Weeks 5-6: Stripe integration and billing system
- Critical path for revenue generation

### Phase 2: Enhanced User Experience (Medium Priority)
**Tasks 4.x, 5.x, 6.x** - Improve usability
- Weeks 7-9: Terminal emulator and multi-provider chat
- Weeks 10-11: Document processing and cloud integration
- Parallel development possible

### Phase 3: Intelligence & Insights (Medium Priority)
**Tasks 7.x & 9.x** - Add smart features
- Weeks 12-13: Web search integration
- Weeks 14-15: Analytics and reporting
- Can be developed independently

### Risk Mitigation
- **Marketplace Complexity:** Implement circuit breaker pattern for failover
- **Payment Security:** Follow Stripe best practices, PCI compliance
- **WebSocket Scale:** Connection pooling, auto-reconnect
- **Cost Tracking:** Double-entry accounting for accuracy
- **Cloud API Limits:** Rate limiting, quota monitoring

### Testing Approach
- **Unit Tests:** Rust tests for all business logic
- **Integration Tests:** API endpoint testing with test database
- **E2E Tests:** Cypress for critical user flows
- **Load Tests:** Artillery for marketplace routing
- **Security Tests:** OWASP ZAP scans, penetration testing

## Task Breakdown (35 Tasks)

### Task 3: API Key Marketplace Core System (HIGH PRIORITY)

**3.1 Marketplace Database Operations Layer**
- Create Rust structs and traits for marketplace entities
- Implement repository pattern for `api_key_listings` table operations
- Add CRUD methods for key listing management
- Write migration scripts for any new tables/indexes
- Test: Unit tests for all database operations

**3.2 API Key Listing Management Endpoints**
- Build REST endpoints: POST/GET/PUT/DELETE `/api/marketplace/listings`
- Implement listing creation with validation
- Add listing search and filter capabilities
- Create provider dashboard endpoint
- Test: Integration tests for CRUD operations

**3.3 Dynamic Pricing Engine**
- Implement base rate multiplier calculations
- Add pricing strategy configuration per listing
- Create cost estimation endpoint for buyers
- Build real-time pricing updates
- Test: Unit tests for pricing accuracy

**3.4 Capacity Tracking System**
- Implement usage monitoring per API key
- Add daily/monthly limit enforcement
- Create capacity status tracking
- Build automatic key deactivation on limit
- Test: Load tests for concurrent usage tracking

**3.5 Intelligent Request Routing**
- Build routing algorithm for key selection
- Implement failover to alternative keys
- Add load balancing across available keys
- Create routing metrics and logging
- Test: Integration tests for routing scenarios

---

### Task 4: Terminal Emulator with Matrix Styling (MEDIUM PRIORITY)

**4.1 xterm.js Integration**
- Install and configure xterm.js library
- Create terminal container in web UI
- Apply Matrix theme (green text, terminal fonts)
- Add terminal addons (fit, web-links)
- Test: Browser compatibility testing

**4.2 WebSocket Terminal Backend**
- Create WebSocket endpoint `/ws/terminal` in `src/serve.rs`
- Implement WebSocket upgrade handler
- Add command message parsing
- Build response streaming to terminal
- Test: WebSocket connection stability

**4.3 CLI Command Routing**
- Route terminal commands to aichat CLI engine
- Implement command execution with output capture
- Add command history and autocomplete support
- Create sandboxed execution environment
- Test: E2E CLI command execution

**4.4 Session Synchronization**
- Sync terminal session with GUI session
- Implement bi-directional state updates
- Add session restoration on reconnect
- Create shared context management
- Test: Session state consistency validation

**4.5 Terminal Performance Optimization**
- Implement terminal output buffering
- Add virtual scrolling for long output
- Optimize WebSocket message batching
- Create terminal rendering performance metrics
- Test: Performance benchmarks

---

### Task 5: Enhanced Multi-Provider Chat Interface (MEDIUM PRIORITY)

**5.1 Side-by-Side UI Layout**
- Design responsive multi-column layout
- Implement provider selection checkboxes
- Create per-provider response containers
- Add Matrix-themed styling
- Test: Responsive layout across devices

**5.2 Concurrent Provider Requests**
- Implement parallel API calls to multiple providers
- Add streaming response handlers per provider
- Create request queuing and retry logic
- Build error handling per provider
- Test: Concurrent request performance

**5.3 Real-Time Cost Tracking**
- Calculate cost per provider in real-time
- Display running total during conversation
- Add cost comparison visualization
- Create cost alert thresholds
- Test: Cost calculation accuracy

**5.4 Marketplace Key Routing Integration**
- Connect multi-provider UI to marketplace routing
- Implement automatic failover display
- Add marketplace key status indicators
- Create provider availability monitoring
- Test: Marketplace routing integration

**5.5 Response Comparison Features**
- Add response highlighting and diff view
- Implement side-by-side scrolling sync
- Create response quality voting
- Build response export functionality
- Test: UI/UX usability testing

---

### Task 6: Document Processing and Cloud Integration (MEDIUM PRIORITY)

**6.1 Google Drive API Integration**
- Implement OAuth2 flow for Google Drive
- Create Drive file browser UI
- Add file download and caching
- Build Drive file metadata extraction
- Test: Google Drive API integration

**6.2 OneDrive API Integration**
- Implement OAuth2 flow for OneDrive
- Create OneDrive file browser UI
- Add file download and caching
- Build OneDrive file metadata extraction
- Test: OneDrive API integration

**6.3 Unified Cloud File Browser**
- Create unified UI for all cloud sources
- Implement file type filtering
- Add thumbnail previews
- Build drag-and-drop upload interface
- Test: Cloud browser UI testing

**6.4 Document Processing Pipeline**
- Extend `src/rag/` for cloud files
- Implement chunking for large documents
- Add progress tracking for processing
- Create document status management
- Test: Document processing validation

**6.5 RAG Chat Integration**
- Integrate processed docs into chat context
- Add document reference citations
- Create document-aware prompting
- Build document removal from context
- Test: RAG accuracy testing

---

### Task 7: Web Search Integration (LOW PRIORITY)

**7.1 DuckDuckGo API Client**
- Implement DuckDuckGo Instant Answer API
- Create search result parsing
- Add result caching
- Build search query optimization
- Test: DuckDuckGo API integration

**7.2 Google Search API Client**
- Implement Google Custom Search API
- Create premium search result parsing
- Add API key management
- Build search quota monitoring
- Test: Google Search API integration

**7.3 Search Result Processing**
- Implement citation extraction
- Add relevance scoring algorithm
- Create result deduplication
- Build snippet extraction
- Test: Search result accuracy

**7.4 Chat Context Integration**
- Add search results to chat context
- Implement automatic search triggers
- Create citation display in responses
- Build source link formatting
- Test: Context integration validation

**7.5 Search Fallback Mechanisms**
- Implement multi-provider fallback
- Add graceful degradation
- Create offline mode handling
- Build search unavailable notifications
- Test: Fallback scenario testing

---

### Task 8: Billing and Payment System (HIGH PRIORITY)

**8.1 Stripe API Integration**
- Set up Stripe API client
- Implement payment intent creation
- Add payment method management
- Build Stripe customer sync
- Test: Stripe API integration

**8.2 Payment Webhook Handler**
- Create webhook endpoint `/api/webhooks/stripe`
- Implement webhook signature verification
- Add payment confirmation processing
- Build failed payment handling
- Test: Webhook security validation

**8.3 User Balance Management**
- Implement balance tracking system
- Add transaction history recording
- Create balance update atomicity
- Build balance inquiry endpoints
- Test: Balance accuracy testing

**8.4 Billing Dashboard UI**
- Create transaction history view
- Implement payment method management UI
- Add spending analytics visualizations
- Build Matrix-themed dashboard
- Test: Dashboard UI/UX testing

**8.5 Payout System**
- Implement payout request processing
- Add platform fee calculation
- Create payout scheduling
- Build payout history tracking
- Test: Payout system verification

---

### Task 9: Analytics and Reporting System (MEDIUM PRIORITY)

**9.1 Analytics Data Collection**
- Implement usage tracking middleware
- Add event logging for all user actions
- Create marketplace transaction logging
- Build performance metrics collection
- Test: Data collection accuracy

**9.2 Database Schema for Analytics**
- Create `usage_analytics` table migration
- Add `system_metrics` table migration
- Implement analytics data models
- Build analytics indexes for performance
- Test: Schema validation

**9.3 Aggregation and Reporting Logic**
- Implement daily/weekly/monthly aggregations
- Add cost breakdown calculations
- Create trend analysis algorithms
- Build report scheduling
- Test: Aggregation correctness

**9.4 User Analytics Dashboard**
- Create usage statistics UI
- Implement cost tracking visualizations
- Add trend charts and graphs
- Build Matrix-themed dashboard
- Test: Dashboard data accuracy

**9.5 Admin Analytics & PDF Reports**
- Implement system-wide analytics dashboard
- Add marketplace metrics views
- Create PDF report generation
- Build scheduled report delivery
- Test: Report generation performance

---

## Dependencies

### External Service Dependencies
- **Supabase/PostgreSQL:** Database (already configured)
- **Stripe:** Payment processing (API key required)
- **Google Drive API:** Cloud storage (OAuth setup required)
- **OneDrive API:** Cloud storage (OAuth setup required)
- **DuckDuckGo API:** Web search (free tier)
- **Google Custom Search:** Premium search (API key required)
- **OpenRouter + 20+ LLM Providers:** Already integrated

### Internal Dependencies
- **Completed Foundation:** Tasks 1-2 (Matrix theme, Auth) ✅
- **Marketplace → Billing:** Task 8 depends on Task 3
- **Multi-Provider → Marketplace:** Task 5 depends on Task 3
- **Session Mgmt:** Task 4 can leverage existing `src/config/session.rs`
- **Document Processing:** Task 6 can leverage existing `src/rag/`

### Prerequisite Work
- ✅ Database schema migrated
- ✅ JWT authentication implemented
- ✅ API key encryption utilities ready
- ✅ Matrix theme CSS framework complete
- 🔄 Stripe account setup needed
- 🔄 Google/OneDrive OAuth apps needed

## Success Criteria (Technical)

### Performance Benchmarks
- **API Response Time:** < 200ms for 95th percentile
- **WebSocket Latency:** < 50ms for terminal commands
- **Marketplace Routing:** < 100ms for key selection
- **Database Queries:** < 50ms for common operations
- **Document Processing:** < 5s for typical PDFs
- **Analytics Aggregation:** < 2s for daily reports

### Quality Gates
- **Code Coverage:** > 80% for business logic
- **Security Scan:** Zero high/critical vulnerabilities
- **Load Testing:** Handle 1000 concurrent users
- **Uptime:** 99.9% availability for marketplace
- **Error Rate:** < 0.1% for critical paths
- **Accessibility:** WCAG 2.1 AA compliance

### Acceptance Criteria
- All 35 subtasks completed and tested
- Integration tests passing for all features
- Security audit completed and issues resolved
- Performance benchmarks met
- Documentation complete (API docs, user guides)
- Production deployment successful

## Estimated Effort

### Overall Timeline
- **Total Duration:** 15 weeks (3.5 months)
- **Phase 1 (Marketplace & Payments):** 6 weeks
- **Phase 2 (UX Enhancements):** 5 weeks
- **Phase 3 (Intelligence & Insights):** 4 weeks

### Resource Requirements
- **Backend Developer:** 1 FTE (Rust expertise)
- **Frontend Developer:** 0.5 FTE (JavaScript/Matrix UI)
- **DevOps:** 0.25 FTE (deployment, monitoring)
- **QA/Testing:** 0.5 FTE (throughout project)

### Critical Path Items
1. **Task 3.1-3.5:** Marketplace core (Week 1-4) - CRITICAL
2. **Task 8.1-8.3:** Billing system (Week 5-6) - CRITICAL
3. **Task 3.5 + 5.4:** Routing integration (Week 9) - CRITICAL
4. **Security & Load Testing:** (Week 14-15) - CRITICAL

### Task Complexity Breakdown
- **High Complexity (2-3 weeks each):** Tasks 3.5, 8.1-8.3
- **Medium Complexity (1-2 weeks each):** Tasks 4.2-4.4, 5.2-5.4, 6.4-6.5
- **Low Complexity (< 1 week each):** Tasks 7.1-7.5, 9.1-9.5

### Risk Contingency
- **Buffer Time:** 2 weeks added for unforeseen issues
- **Integration Challenges:** Plan for 1 week integration testing
- **Third-party API Issues:** Fallback implementations ready

## Tasks Created

### Task 3: API Key Marketplace Core System
- [ ] 002.md - Marketplace Database Operations Layer (M, 40h, parallel: true)
- [ ] 003.md - API Key Listing Management Endpoints (M, 32h, parallel: false, depends: 002)
- [ ] 004.md - Dynamic Pricing Engine (M, 24h, parallel: true, depends: 002, conflicts: 003)
- [ ] 005.md - Capacity Tracking System (M, 32h, parallel: true, depends: 002, conflicts: 003)
- [ ] 006.md - Intelligent Request Routing (L, 48h, parallel: false, depends: 002,003,004,005)

### Task 4: Terminal Emulator with Matrix Styling
- [ ] 007.md - xterm.js Integration (S, 16h, parallel: true)
- [ ] 008.md - WebSocket Terminal Backend (M, 32h, parallel: true)
- [ ] 009.md - CLI Command Routing (M, 32h, parallel: false, depends: 008)
- [ ] 010.md - Session Synchronization (M, 32h, parallel: true, depends: 008, conflicts: 009)
- [ ] 011.md - Terminal Performance Optimization (S, 16h, parallel: false, depends: 008,009)

### Task 5: Enhanced Multi-Provider Chat Interface
- [ ] 012.md - Side-by-Side UI Layout (S, 16h, parallel: true)
- [ ] 013.md - Concurrent Provider Requests (M, 40h, parallel: true)
- [ ] 014.md - Real-Time Cost Tracking (S, 16h, parallel: false, depends: 013)
- [ ] 015.md - Marketplace Key Routing Integration (M, 24h, parallel: false, depends: 006,013)
- [ ] 016.md - Response Comparison Features (S, 16h, parallel: true, depends: 013, conflicts: 014,015)

### Task 6: Document Processing and Cloud Integration
- [ ] 017.md - Google Drive API Integration (M, 32h, parallel: true)
- [ ] 018.md - OneDrive API Integration (M, 32h, parallel: true, conflicts: 017)
- [ ] 019.md - Unified Cloud File Browser (S, 16h, parallel: false, depends: 017,018)
- [ ] 020.md - Document Processing Pipeline (M, 40h, parallel: false, depends: 019)
- [ ] 021.md - RAG Chat Integration (M, 32h, parallel: false, depends: 020)

### Task 7: Web Search Integration
- [ ] 022.md - DuckDuckGo API Client (S, 16h, parallel: true)
- [ ] 023.md - Google Search API Client (S, 16h, parallel: true, conflicts: 022)
- [ ] 024.md - Search Result Processing (S, 16h, parallel: false, depends: 022,023)
- [ ] 025.md - Chat Context Integration (M, 24h, parallel: false, depends: 024)
- [ ] 026.md - Search Fallback Mechanisms (S, 8h, parallel: false, depends: 022,023,025)

### Task 8: Billing and Payment System
- [ ] 027.md - Stripe API Integration (M, 32h, parallel: true)
- [ ] 028.md - Payment Webhook Handler (M, 24h, parallel: false, depends: 027)
- [ ] 029.md - User Balance Management (M, 32h, parallel: true, depends: 027, conflicts: 028)
- [ ] 030.md - Billing Dashboard UI (M, 24h, parallel: false, depends: 027,028,029)
- [ ] 031.md - Payout System (M, 32h, parallel: false, depends: 027,029)

### Task 9: Analytics and Reporting System
- [ ] 032.md - Analytics Data Collection (M, 24h, parallel: true)
- [ ] 033.md - Database Schema for Analytics (S, 16h, parallel: true, conflicts: 032)
- [ ] 034.md - Aggregation and Reporting Logic (M, 32h, parallel: false, depends: 032,033)
- [ ] 035.md - User Analytics Dashboard (M, 24h, parallel: false, depends: 034)
- [ ] 036.md - Admin Analytics & PDF Reports (M, 32h, parallel: true, depends: 034, conflicts: 035)

---

## Task Summary Statistics

**Total Tasks:** 35
**Parallel Tasks:** 16 (can start immediately or run in parallel)
**Sequential Tasks:** 19 (have dependencies)
**Total Estimated Effort:** 968 hours (~24 weeks at 1 FTE)

**Effort Breakdown by Size:**
- Small (S): 12 tasks, 184 hours
- Medium (M): 22 tasks, 736 hours
- Large (L): 1 task, 48 hours

**Effort Breakdown by Feature:**
- Task 3 (Marketplace): 176 hours
- Task 4 (Terminal): 128 hours
- Task 5 (Multi-Provider): 112 hours
- Task 6 (Documents): 152 hours
- Task 7 (Search): 80 hours
- Task 8 (Billing): 144 hours
- Task 9 (Analytics): 128 hours

---

**Epic Status:** In Progress (13% complete)
**Next Milestone:** Complete Task 3 (Marketplace Core) by Week 4
**Last Updated:** 2025-10-06T13:16:05Z
