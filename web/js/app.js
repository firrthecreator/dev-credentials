(function() {
  let registry = {};
  let toastTimer = null;

  function initTheme() {
    const saved = localStorage.getItem("dev-theme") || "dark";
    setTheme(saved);

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
  }

  function showToast(text) {
    const toast = document.getElementById("toast");
    if (!toast) return;
    toast.textContent = text;
    toast.classList.add("show");
    if (toastTimer) clearTimeout(toastTimer);
    toastTimer = setTimeout(() => {
      toast.classList.remove("show");
    }, 1600);
  }

  function copyText(text, label) {
    if (!text) return;
    navigator.clipboard.writeText(text).then(() => {
      showToast(label ? `Copied ${label}` : "Copied to clipboard");
    }).catch(() => {
      showToast("Unable to copy");
    });
  }

  async function loadRegistry() {
    if (window.__DEFAULT_REGISTRY__) {
      registry = Object.assign({}, window.__DEFAULT_REGISTRY__);
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

  function escapeHtml(str) {
    if (!str) return "";
    return String(str)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;")
      .replace(/'/g, "&#039;");
  }

  function findMatches(query) {
    const clean = (query || "").trim().toLowerCase();
    if (!clean) return [];

    const matches = [];
    for (const key of Object.keys(registry)) {
      const item = registry[key];
      if (
        item.id.toLowerCase() === clean ||
        item.id.toLowerCase().includes(clean) ||
        item.legality_id.toLowerCase().includes(clean) ||
        item.name.toLowerCase().includes(clean) ||
        String(item.member_no) === clean ||
        `#${item.member_no}` === clean
      ) {
        matches.push(item);
      }
    }
    matches.sort((a, b) => (a.member_no || 0) - (b.member_no || 0));
    return matches;
  }

  function renderEmptyPrompt() {
    const container = document.getElementById("result-view");
    container.innerHTML = `
      <div class="card state-empty">
        Enter a Credential ID, Legality ID, or member name above to inspect authenticity.
      </div>
    `;
  }

  function renderNotFound(query) {
    const container = document.getElementById("result-view");
    container.innerHTML = `
      <div class="card state-empty">
        No credential found matching "${escapeHtml(query)}".
      </div>
    `;
  }

  function renderSearchResults(records) {
    const container = document.getElementById("result-view");
    const rows = records.map(r => `
      <div class="directory-row" data-id="${escapeHtml(r.id)}">
        <div class="row-left">
          <span class="row-num">#${String(r.member_no).padStart(2, "0")}</span>
          <span class="row-name">${escapeHtml(r.name)}</span>
        </div>
        <div class="row-right">
          <span class="row-id">${escapeHtml(r.id)}</span>
          <svg class="row-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="m9 18 6-6-6-6"/>
          </svg>
        </div>
      </div>
    `).join("");

    container.innerHTML = `
      <div class="directory-header">
        <span class="directory-title">Matching Results</span>
        <span class="directory-count">${records.length} ${records.length === 1 ? "match" : "matches"}</span>
      </div>
      <div class="directory-list">
        ${rows}
      </div>
    `;

    container.querySelectorAll(".directory-row").forEach(row => {
      row.addEventListener("click", function() {
        const id = this.getAttribute("data-id");
        if (id && registry[id]) {
          const input = document.getElementById("search-input");
          if (input) input.value = id;
          renderDetail(registry[id]);
          history.replaceState(null, "", `?id=${encodeURIComponent(id)}`);
        }
      });
    });
  }

  function renderDetail(record) {
    const container = document.getElementById("result-view");
    const num = String(record.member_no || "").padStart(2, "0");

    container.innerHTML = `
      <div class="card">
        <div class="card-header">
          <div class="recipient-meta">
            <span class="recipient-num">MEMBER #${num}</span>
            <h2 class="recipient-name">${escapeHtml(record.name)}</h2>
          </div>
          <span class="badge-verified">
            <span class="badge-dot"></span>
            Authentic
          </span>
        </div>

        <div class="info-grid">
          <div class="info-item">
            <span class="info-label">Credential ID</span>
            <div class="info-value-wrap">
              <span class="info-value mono">${escapeHtml(record.id)}</span>
              <button type="button" class="btn-copy-mini" data-copy="${escapeHtml(record.id)}" data-label="Credential ID" aria-label="Copy ID">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <rect width="14" height="14" x="8" y="8" rx="2" ry="2"/>
                  <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>
                </svg>
              </button>
            </div>
          </div>

          <div class="info-item">
            <span class="info-label">Legality ID</span>
            <div class="info-value-wrap">
              <span class="info-value mono">${escapeHtml(record.legality_id)}</span>
              <button type="button" class="btn-copy-mini" data-copy="${escapeHtml(record.legality_id)}" data-label="Legality ID" aria-label="Copy Legality ID">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <rect width="14" height="14" x="8" y="8" rx="2" ry="2"/>
                  <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>
                </svg>
              </button>
            </div>
          </div>

          <div class="info-item">
            <span class="info-label">Issue Date</span>
            <span class="info-value mono">${escapeHtml(record.date)}</span>
          </div>

          <div class="info-item">
            <span class="info-label">Algorithm</span>
            <span class="info-value mono">${escapeHtml(record.algorithm)}</span>
          </div>
        </div>

        <details class="details-block">
          <summary class="details-summary">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="m9 18 6-6-6-6"/>
            </svg>
            Cryptographic Audit Details
          </summary>
          <div class="details-content">
            <div class="info-item">
              <span class="info-label">Authority Fingerprint</span>
              <span class="info-value mono">${escapeHtml(record.key_fingerprint)}</span>
            </div>
            <div class="info-item">
              <span class="info-label">Signature SHA3-256 Digest</span>
              <span class="info-value mono">${escapeHtml(record.signature_sha3_256)}</span>
            </div>
            <div class="info-item">
              <span class="info-label">Canonical Audit Payload</span>
              <div class="code-field">${escapeHtml(record.canonical_payload)}</div>
            </div>
          </div>
        </details>

        <div class="card-actions">
          <button type="button" class="btn-ghost" id="btn-copy-url">Copy Link</button>
          <button type="button" class="btn-ghost" id="btn-copy-payload">Copy Payload</button>
          <button type="button" class="btn-ghost" id="btn-copy-json">Copy JSON</button>
        </div>
      </div>
    `;

    container.querySelectorAll(".btn-copy-mini").forEach(btn => {
      btn.addEventListener("click", function() {
        const val = this.getAttribute("data-copy");
        const lbl = this.getAttribute("data-label");
        copyText(val, lbl);
      });
    });

    const btnUrl = document.getElementById("btn-copy-url");
    if (btnUrl) {
      btnUrl.addEventListener("click", () => {
        const url = `${window.location.origin}/verify?id=${encodeURIComponent(record.id)}`;
        copyText(url, "verification link");
      });
    }

    const btnPayload = document.getElementById("btn-copy-payload");
    if (btnPayload) {
      btnPayload.addEventListener("click", () => {
        copyText(record.canonical_payload, "canonical payload");
      });
    }

    const btnJson = document.getElementById("btn-copy-json");
    if (btnJson) {
      btnJson.addEventListener("click", () => {
        copyText(JSON.stringify(record, null, 2), "JSON record");
      });
    }
  }

  function handleQuery(query) {
    const clean = (query || "").trim();
    const clearBtn = document.getElementById("search-clear");
    if (clearBtn) {
      clearBtn.style.display = clean.length ? "flex" : "none";
    }

    if (!clean) {
      renderEmptyPrompt();
      return;
    }

    const matches = findMatches(clean);
    if (matches.length === 1) {
      renderDetail(matches[0]);
    } else if (matches.length > 1) {
      renderSearchResults(matches);
    } else {
      renderNotFound(clean);
    }
  }

  async function init() {
    initTheme();
    await loadRegistry();

    const input = document.getElementById("search-input");
    const clearBtn = document.getElementById("search-clear");

    if (input) {
      input.addEventListener("input", function() {
        handleQuery(this.value);
      });

      input.addEventListener("keydown", function(e) {
        if (e.key === "Escape") {
          this.value = "";
          handleQuery("");
        }
      });
    }

    if (clearBtn && input) {
      clearBtn.addEventListener("click", function() {
        input.value = "";
        input.focus();
        handleQuery("");
      });
    }

    const params = new URLSearchParams(window.location.search);
    const initialQuery = params.get("id") || params.get("verify") || params.get("q");

    if (initialQuery) {
      if (input) input.value = initialQuery;
      handleQuery(initialQuery);
    } else {
      renderEmptyPrompt();
    }
  }

  window.addEventListener("DOMContentLoaded", init);
})();
