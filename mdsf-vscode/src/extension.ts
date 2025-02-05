import { spawn } from "node:child_process";
import * as vscode from "vscode";

// This extension is loosely based on the vscode extension created by JohnnyMorganz for Stylua
// https://github.com/JohnnyMorganz/StyLua/blob/main/stylua-vscode

const SUPPORTED_DOCUMENT_TYPES = ["markdown"];

const COMMAND_SHOW_TERMINAL_OUTPUT = "mdsf.showTerminalOutput";

enum Status {
  Ready = 0,
  Running = 1,
  Error = 2,
}

class StatusItem implements vscode.Disposable {
  statusItem!: vscode.LanguageStatusItem;

  constructor() {
    this.statusItem = vscode.languages.createLanguageStatusItem(
      "mdsf",
      SUPPORTED_DOCUMENT_TYPES,
    );

    this.statusItem.name = "mdsf";
    this.statusItem.text = "mdsf";

    this.statusItem.command = {
      title: "Show mdsf output",
      command: COMMAND_SHOW_TERMINAL_OUTPUT,
    };

    this.setFormattingStatus(Status.Ready);
  }

  setFormattingStatus(state: Status) {
    switch (state) {
      case Status.Running: {
        this.statusItem.detail = "Running mdsf";
        this.statusItem.severity = vscode.LanguageStatusSeverity.Information;
        break;
      }

      case Status.Ready: {
        this.statusItem.detail = "Ready to format";
        this.statusItem.severity = vscode.LanguageStatusSeverity.Information;
        break;
      }

      case Status.Error: {
        this.statusItem.detail = "Failed to format file";
        this.statusItem.severity = vscode.LanguageStatusSeverity.Error;
        break;
      }
    }
  }

  dispose() {
    this.statusItem.dispose();
  }
}

async function formatFile(filePath: string, cwd?: string | URL | undefined) {
  return new Promise<string>((resolve, reject) => {
    const p = spawn("mdsf", ["format", filePath], { cwd });

    let output = "";

    p.stdout.on("data", (data) => {
      output += data.toString();
    });

    p.stdout.on("close", () => {
      resolve(output);
    });

    p.stderr.on("data", (data) => {
      output += data.toString();
    });

    p.on("err", (err) => {
      console.error("format error", err);

      reject("Failed to start mdsf");
    });
  });
}

export function activate(context: vscode.ExtensionContext) {
  console.log("mdsf activated");

  const statusItem = new StatusItem();

  const outputChannel = vscode.window.createOutputChannel("mdsf", {
    log: true,
  });
  outputChannel.info("mdsf activated");

  context.subscriptions.push(
    vscode.commands.registerCommand(COMMAND_SHOW_TERMINAL_OUTPUT, async () => {
      outputChannel.show();
    }),
  );

  const disposable = vscode.languages.registerDocumentFormattingEditProvider(
    SUPPORTED_DOCUMENT_TYPES,
    {
      async provideDocumentFormattingEdits(document, _options, _token) {
        statusItem.setFormattingStatus(Status.Running);

        await formatFile(
          document.fileName,
          vscode.workspace.getWorkspaceFolder(document.uri)?.uri?.fsPath,
        )
          .then((stdout) => {
            console.info("stdout", stdout);
            outputChannel.info(stdout);
            statusItem.setFormattingStatus(Status.Ready);
          })
          .catch((error) => {
            console.info("error", error);

            outputChannel.error(error);
            statusItem.setFormattingStatus(Status.Error);
          });

        return [];
      },
    },
  );

  context.subscriptions.push(statusItem);

  context.subscriptions.push(disposable);

  vscode.window.onDidChangeActiveTextEditor((_editor) => {
    statusItem.setFormattingStatus(Status.Ready);
  });
}

export function deactivate() {}
