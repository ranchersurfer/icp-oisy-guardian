import path from 'path'
import os from 'os'

export const WORKSPACE_ROOT = path.join(os.homedir(), '.openclaw', 'workspace')

export function workspacePath(...segments: string[]): string {
  return path.join(WORKSPACE_ROOT, ...segments)
}
