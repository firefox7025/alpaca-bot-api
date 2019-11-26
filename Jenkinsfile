podTemplate(yaml: """
apiVersion: v1
kind: Pod
metadata:
  labels:
    some-label: builder
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
    stage('Build and test') {
    checkout scm
      container('rust') {
        sh 'cargo build --release && cargo test'
      }
    }
  }
}