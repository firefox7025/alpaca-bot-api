podTemplate(yaml: """
apiVersion: v1
kind: Pod
metadata:
  labels:
    some-label: some-label-value
spec:
  containers:
  - name: rust
    image: rust:1.39.0-stretch
    command:
    - cat
    tty: true
"""
  ) {

  node(POD_LABEL) {
    stage('Build') {
        git 'https://github.com/firefox7025/alpaca-bot-api.git'
        git branch: 'branchName', credentialsId: 'your_credentials', url: "giturlrepo"
      container('rust') {
        sh 'echo ${GIT_BRANCH}'
        sh 'cargo build --release'
      }
    }
  }
}