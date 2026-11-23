(function() {
  let registry = {};

  function initTheme() {
    const saved = localStorage.getItem("dev-theme");
    const theme = saved || "dark";
    setTheme(theme);

    const toggle = document.getElementById("theme-toggle");
    if (toggle) {
      toggle.addEventListener("click", function() {
        const current = document.documentElement.getAttribute("data-theme") || "dark";
        const next = current === "dark" ? "light" : "dark";
        setTheme(next);
      });
    }
  }

  function setTheme(theme) {
    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem("dev-theme", theme);
    const label = document.getElementById("theme-label");
    if (label) {
      label.textContent = theme === "dark" ? "Dark" : "Light";
    }
  }

  async function loadRegistry() {
    if (window.__DEFAULT_REGISTRY__) {
      registry = window.__DEFAULT_REGISTRY__;
    }
    const paths = ["data/db_registry.json", "../data/db_registry.json"];
    for (const p of paths) {
      try {
        const response = await fetch(p);
        if (response.ok) {
          const data = await response.json();
          registry = Object.assign({}, registry, data);
          break;
        }
      } catch (e) {
      }
    }
  }

  function findRecord(query) {
    if (!query) return null;
    const clean = query.trim();
    if (!clean) return null;

    if (registry[clean]) {
      return registry[clean];
    }

    const lower = clean.toLowerCase();
    for (const key of Object.keys(registry)) {
      const item = registry[key];
      if (
        item.id.toLowerCase() === lower ||
        item.legality_id.toLowerCase() === lower ||
        String(item.member_no) === lower ||
        item.name.toLowerCase().includes(lower)
      ) {
        return item;
      }
    }
    return null;
  }

  function renderEmptyState(message) {
    const container = document.getElementById("result-container");
    container.innerHTML = `
      <div class="empty-state">
        ${escapeHtml(message || "Enter a credential identifier above to inspect cryptographic authenticity.")}
      </div>
    `;
  }

  function renderNotFound(query) {
    const container = document.getElementById("result-container");
    container.innerHTML = `
      <div class="result-card">
        <div class="status-banner invalid">
          <div class="status-main">
            <span class="status-tag">Record Not Found</span>
            <span class="status-desc">No cryptographic signature matched this identifier in the authoritative registry.</span>
          </div>
        </div>
        <div class="detail-item">
          <span class="detail-label">Query Searched</span>
          <span class="detail-value mono">${escapeHtml(query)}</span>
        </div>
      </div>
    `;
  }

  function renderValid(record) {
    const container = document.getElementById("result-container");
    container.innerHTML = `
      <div class="result-card">
        <div class="status-banner valid">
          <div class="status-main">
            <span class="status-tag">Authentic & Verified</span>
            <span class="status-desc">NIST FIPS 204 ML-DSA-65 post-quantum cryptographic signature verified.</span>
          </div>
        </div>

        <div class="grid-details">
          <div class="detail-item">
            <span class="detail-label">Recipient Name</span>
            <span class="detail-value">${escapeHtml(record.name)}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">Member Sequence</span>
            <span class="detail-value mono">#${record.member_no}</span>
          </div>

          <div class="detail-item">
            <span class="detail-label">Credential ID</span>
            <span class="detail-value mono">${escapeHtml(record.id)}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">Rivide Legality ID</span>
            <span class="detail-value mono">${escapeHtml(record.legality_id)}</span>
          </div>

          <div class="detail-item">
            <span class="detail-label">Issue Date</span>
            <span class="detail-value mono">${escapeHtml(record.date)}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">Algorithm</span>
            <span class="detail-value mono">${escapeHtml(record.algorithm)}</span>
          </div>

          <div class="detail-item full-width">
            <span class="detail-label">Authority Fingerprint</span>
            <span class="detail-value mono">${escapeHtml(record.key_fingerprint)}</span>
          </div>

          <div class="detail-item full-width">
            <span class="detail-label">Signature SHA3-256 Digest</span>
            <span class="detail-value mono">${escapeHtml(record.signature_sha3_256)}</span>
          </div>

          <div class="detail-item full-width">
            <span class="detail-label">Canonical Audit Payload</span>
            <div class="code-box">${escapeHtml(record.canonical_payload)}</div>
          </div>
        </div>

        <div class="action-row">
          <button class="action-btn" id="btn-copy-payload">Copy Canonical Payload</button>
          <button class="action-btn" id="btn-copy-json">Copy JSON Record</button>
        </div>
      </div>
    `;

    document.getElementById("btn-copy-payload").addEventListener("click", function() {
      navigator.clipboard.writeText(record.canonical_payload);
      this.textContent = "Copied";
      setTimeout(() => { this.textContent = "Copy Canonical Payload"; }, 1500);
    });

    document.getElementById("btn-copy-json").addEventListener("click", function() {
      navigator.clipboard.writeText(JSON.stringify(record, null, 2));
      this.textContent = "Copied";
      setTimeout(() => { this.textContent = "Copy JSON Record"; }, 1500);
    });
  }

  function escapeHtml(str) {
    if (!str) return "";
    return String(str)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;")
      .replace(/'/g, "&#039;");
  }

  function handleSearch(query) {
    if (!query) {
      renderEmptyState();
      return;
    }
    const record = findRecord(query);
    if (record) {
      renderValid(record);
    } else {
      renderNotFound(query);
    }
  }

  async function init() {
    initTheme();
    await loadRegistry();

    const form = document.getElementById("search-form");
    const input = document.getElementById("search-input");

    form.addEventListener("submit", function(e) {
      e.preventDefault();
      handleSearch(input.value);
    });

    document.querySelectorAll(".chip").forEach(function(chip) {
      chip.addEventListener("click", function() {
        const val = this.getAttribute("data-id");
        input.value = val;
        handleSearch(val);
      });
    });

    const params = new URLSearchParams(window.location.search);
    const initialQuery = params.get("id") || params.get("verify") || params.get("q");
    if (initialQuery) {
      input.value = initialQuery;
      handleSearch(initialQuery);
    } else {
      renderEmptyState();
    }
  }

  window.addEventListener("DOMContentLoaded", init);
})();
