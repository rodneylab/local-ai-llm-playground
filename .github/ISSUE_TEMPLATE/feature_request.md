name: Feature request
description: Share your ideas for new features or improvements
labels: ["feature request"]
body:

- type: markdown
  attributes:
  value: | > [!NOTE] > > Do not prefix your title with "[REQUEST]", "[Feature request]", etc., a label will be added automatically.

      Please provide a detailed description of what the feature would do and what it would look like:

      * What problem would this feature solve?
      * Are there any potential downsides to this feature?
      * If applicable, what would the configuration for this feature look like?
      * Are there any existing examples of this feature in other software?
      * If applicable, include any external documentation required to implement this feature
      * Anything else you think might be relevant

      **No need to copy the above list into your description, it's just a guide to help you provide the most useful information.**

- type: textarea
  id: description
  validations:
  required: true
  attributes:
  label: Description

- type: markdown
  attributes:
  value: |
  Thank you for taking the time to submit your idea.
