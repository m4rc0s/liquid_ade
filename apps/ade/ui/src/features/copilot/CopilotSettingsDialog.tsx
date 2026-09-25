import { useState } from 'react';
import { saveCopilotSettings, type CopilotSettings } from './api';

interface CopilotSettingsDialogProps {
  settings: CopilotSettings | null;
  onClose: () => void;
  onSaved: (settings: CopilotSettings) => void;
}

/** Gemini models the gateway accepts today; the provider slug itself is server-owned. */
const MODEL_OPTIONS = ['gemini-2.5-flash', 'gemini-2.5-pro'];

/**
 * Provider credential dialog.
 *
 * The key is write-only by construction: the field starts empty on every open, the server never
 * returns the stored secret, and leaving it blank keeps the credential already on disk (R3).
 */
export function CopilotSettingsDialog({ settings, onClose, onSaved }: CopilotSettingsDialogProps) {
  const [apiKey, setApiKey] = useState('');
  const [model, setModel] = useState(settings?.model ?? MODEL_OPTIONS[0]);
  const [error, setError] = useState<string | null>(null);
  const [isSaving, setIsSaving] = useState(false);

  const isEnvManaged = settings?.credential_source === 'environment';

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    setError(null);
    setIsSaving(true);

    try {
      const saved = await saveCopilotSettings({
        apiKey: apiKey.trim() === '' ? undefined : apiKey.trim(),
        model,
      });
      setApiKey('');
      onSaved(saved);
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : 'Failed to save co-pilot settings');
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <div className="astryx-modal-overlay">
      <div className="astryx-modal" role="dialog" aria-modal="true" aria-label="Co-Pilot Provider">
        <div className="astryx-modal-header">
          <span className="astryx-modal-title">Co-Pilot Provider</span>
          <button type="button" className="astryx-modal-close" onClick={onClose}>
            ✕
          </button>
        </div>

        <form onSubmit={handleSubmit}>
          <div className="astryx-modal-body">
            {error && <div className="astryx-error-box">{error}</div>}

            <div className="astryx-settings-state">
              <span className="astryx-label">Status</span>
              <span className={`astryx-chip ${settings?.configured ? 'ready' : 'draft'}`}>
                {settings?.configured ? 'Configured' : 'Not configured'}
              </span>
              <span className="astryx-input-hint">
                Provider: {settings?.provider ?? 'gemini'} · Source:{' '}
                {settings?.credential_source ?? 'none'}
              </span>
            </div>

            <div className="astryx-form-group">
              <label htmlFor="copilot-api-key-input" className="astryx-label">
                API Key
              </label>
              <input
                id="copilot-api-key-input"
                type="password"
                className="astryx-input"
                placeholder={settings?.configured ? 'Stored — leave blank to keep' : 'AIza…'}
                value={apiKey}
                onChange={(e) => setApiKey(e.target.value)}
                autoComplete="off"
              />
              <span className="astryx-input-hint">
                Stored in the local SQLite <code>settings</code> table and never returned to the
                browser nor embedded in this bundle.
                {isEnvManaged &&
                  ' The credential currently in force comes from GEMINI_API_KEY; saving one here takes precedence.'}
              </span>
            </div>

            <div className="astryx-form-group">
              <label htmlFor="copilot-model-input" className="astryx-label">
                Model
              </label>
              <select
                id="copilot-model-input"
                className="astryx-input"
                value={model}
                onChange={(e) => setModel(e.target.value)}
              >
                {MODEL_OPTIONS.map((option) => (
                  <option key={option} value={option}>
                    {option}
                  </option>
                ))}
                {!MODEL_OPTIONS.includes(model) && <option value={model}>{model}</option>}
              </select>
              <span className="astryx-input-hint">
                Applies to every subsequent co-pilot turn on this workspace.
              </span>
            </div>
          </div>

          <div className="astryx-modal-footer">
            <button
              type="button"
              className="astryx-btn astryx-btn-secondary"
              onClick={onClose}
              disabled={isSaving}
            >
              Cancel
            </button>
            <button type="submit" className="astryx-btn astryx-btn-primary" disabled={isSaving}>
              {isSaving ? 'Saving…' : 'Save Provider'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
