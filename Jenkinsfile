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
        git branch: GIT_BRANCH, url: GIT_URL
      container('rust') {
        sh 'echo ${GIT_BRANCH}'
        sh 'cargo build --release'
      }
    }
  }
}