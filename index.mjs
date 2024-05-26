#!/usr/bin/env zx

import { execSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import clipboardy from 'clipboardy';

function getCommitId(input) {
  try {
    const matchedCommitFormat = input.match(/^[0-9a-fA-F]{40}$/);
    if (!matchedCommitFormat) {
      return;
    }

    try {
      const command = `git cat-file -t ${String(matchedCommitFormat)}`;
      const output = execSync(command).toString('utf-8');

      if (output.includes('commit')) {
        return String(matchedCommitFormat);
      }

      return;
    } catch (_) {
      return;
    }
  } catch (_) {
    return;
  }
}

function chunkDiff(diff, chunkSize = 1000) {
  let chunks = [];
  for (let i = 0; i < diff.length; i += chunkSize) {
    chunks.push(diff.substring(i, i + chunkSize));
  }
  return chunks;
}

function extractImportantLines(diff) {
  let lines = diff.split('\n');
  let importantLines = [];
  let contextLines = 2; // Number of lines to include before and after changes

  for (let i = 0; i < lines.length; i++) {
    if (lines[i].startsWith('diff ') || lines[i].startsWith('index ') || lines[i].startsWith('--- ') || lines[i].startsWith('+++ ') || lines[i].startsWith('@@ ')) {
      // Always include diff headers and index markers
      importantLines.push(lines[i]);
    } else if (lines[i].startsWith('+') || lines[i].startsWith('-')) {
      // Include changes with context
      for (let j = Math.max(0, i - contextLines); j <= Math.min(lines.length - 1, i + contextLines); j++) {
        importantLines.push(lines[j]);
      }
      i += contextLines; // Skip context lines already added
    }
  }

  return [...new Set(importantLines)].join('\n'); // Remove duplicate lines and join
}

const packageJson = readFileSync(`${process.cwd()}/package.json`);

const {
  author,
  description,
  engines,
  main,
  name,
  repository,
  version,
} = JSON.parse(packageJson);

const context = {
  author,
  description,
  engines,
  main,
  name,
  repository,
  version,
};

const lastArg = process.argv[process.argv.length - 1];
const commitId = getCommitId(lastArg);
const gitCommand = commitId ? `git show ${commitId}` : `git diff --cached`;
const diffBody = execSync(gitCommand).toString('utf-8');

// Example usage
const diffChunks = chunkDiff(diffBody);
const summarizedChunks = diffChunks.map(chunk => extractImportantLines(chunk));
const diffSummary = summarizedChunks.join('\n');

const prompt = [
  `Based on the shortened package.json context provided below: ${JSON.stringify(context)}`,
  `and the following git diff summary: ${diffSummary}`,
  `Please write the ideal git commit command that includes all staged changes.`,
  `The commit message should adhere to the Conventional Commits specification outlined here:`,
  `https://www.conventionalcommits.org/en/v1.0.0/#specification`,
  `Ensure the commit message is human-readable and provides an exhaustive description of the changes made.`,
  `Providing a clear and descriptive commit message is crucial for maintaining project history and facilitating collaboration.`,
].join('\n');

clipboardy.writeSync(prompt);

console.log(prompt);
