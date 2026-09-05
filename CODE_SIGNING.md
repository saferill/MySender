# Code signing policy

Free code signing provided by [SignPath.io](https://signpath.io/), certificate by [SignPath Foundation](https://signpath.org/).

Official Windows release artifacts (portable ZIP, EXE installer, MSIX helper) are built automatically from the source code in the [MySender repository](https://github.com/saferill/MySender) using GitHub Actions ([release workflow](.github/workflows/release_v1.yml)) and signed through SignPath. Every signing request is manually approved by an approver listed below.

Only binaries built by MySender are signed. Third-party binaries that are packaged with the app are distributed as provided by their upstream projects and do not receive the MySender signing operation.

Signed Windows files show **SignPath Foundation** as the publisher because the certificate belongs to the foundation, not to the MySender project.

## Team roles

- Committers and reviewers: [@saferill](https://github.com/saferill)
- Approvers: [@saferill](https://github.com/saferill)

## Privacy

MySender's privacy policy is available at [github.com/saferill/MySender/blob/main/PRIVACY.md](https://github.com/saferill/MySender/blob/main/PRIVACY.md).

