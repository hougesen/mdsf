import * as assert from "node:assert";
import * as vscode from "vscode";
import { Status, StatusItem } from "../extension";

suite("StatusItem", () => {
  test("Initial status should match Status.Ready", () => {
    const statusItem = new StatusItem();

    assert.equal(statusItem.languageStatusItem.detail, "Ready to format");

    assert.equal(
      statusItem.languageStatusItem.severity,
      vscode.LanguageStatusSeverity.Information,
    );

    statusItem.dispose();
  });

  suite("#setFormattingStatus", () => {
    test("should support Status.Ready", () => {
      const statusItem = new StatusItem();

      statusItem.setFormattingStatus(Status.Ready);

      assert.equal(statusItem.languageStatusItem.detail, "Ready to format");

      assert.equal(
        statusItem.languageStatusItem.severity,
        vscode.LanguageStatusSeverity.Information,
      );

      statusItem.dispose();
    });

    test("should support Status.Running", () => {
      const statusItem = new StatusItem();

      statusItem.setFormattingStatus(Status.Running);

      assert.equal(statusItem.languageStatusItem.detail, "Running mdsf");
      assert.equal(
        statusItem.languageStatusItem.severity,
        vscode.LanguageStatusSeverity.Information,
      );

      statusItem.dispose();
    });

    test("should suport Status.Error", () => {
      const statusItem = new StatusItem();

      statusItem.setFormattingStatus(Status.Error);

      assert.equal(
        statusItem.languageStatusItem.detail,
        "Failed to format file",
      );
      assert.equal(
        statusItem.languageStatusItem.severity,
        vscode.LanguageStatusSeverity.Error,
      );

      statusItem.dispose();
    });
  });
});
