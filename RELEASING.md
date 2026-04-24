> [!NOTE]
> These steps are aimed at a maintainer releasing a new version

 1. Determine the new version, following [semver](https://semver.org/) appropriately. Make sure you closely examine and understand all commits since the last release.
 2. Create a commit which bumps all versions (using `uv run tools/bump_version.py bump <NEW_VERSION>`). In the same commit, update `ChangeLog.md` to contain human-facing information about changes in the new version; especially make sure to call out any breaking changes. Ideally, people will have been adding notes to `ChangeLog.md` as they make substantive changes, so this step will just be organization. Get that commit reviewed and landed, and note the SHA once it's landed onto `main`.
 3. [Create a new GitHub Release](https://github.com/svix/diom/releases/new) pointing at the SHA from step 2. Mark it as a "draft". Copy the text from your `ChangeLog.md` entry into the release body under a `# Changes` header. Make sure to create a tag with an appropriate name (this should be your version number preceded by the character `v`; e.g., `v1.2.3`)
 4. Invoke the [release.yml workflow](https://github.com/svix/diom/actions/workflows/release.yml) with the tag name you created in Step 3. This will prompt other members of @svix/Maintainers to approve the release, and then will build various artifacts
 5. Invoke the [mega releaser workflow](https://github.com/svix/diom/actions/workflows/mega-releaser.yml) with the tag name you created in Step 3. It is normal for the Maven upload to take approximately 20 minutes.
 6. Create a new commit which adds an `## Unreleased` section to the top of `ChangeLog.md` for future commits to record information

### Writing good Changelogs

A ChangeLog shouldn't just be a list of git commit messages; it should summarize the most important things that end users and other developers need to know about a given release.

Some miscellaneous rules:

 - Each entry should be a &lt;h2&gt; (`##` in Markdown) beginning with the word "Version"
 - Put server updates first, then libraries/clients, then miscellany
 - Don't forget to thank new external contributors!
