name: Bug Report
description: Create a report to help us fix an issue
title: "fix: "
labels: ["bug"]
body:
  - type: markdown
    attributes:
      value: |
        Thanks for filing a bug report.
  - type: textarea
    id: description
    attributes:
      label: Description
      description: A clear and concise description of the bug.
    validations:
      required: true
  - type: textarea
    id: reproduction
    attributes:
      label: Steps to Reproduce
      description: How to trigger the issue.
    validations:
      required: true
  - type: input
    id: os_version
    attributes:
      label: Windows Version & Display Scaling
      description: e.g. Windows 11 23H2, 1440p @ 125% DPI
    validations:
      required: true
