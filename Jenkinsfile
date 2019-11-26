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
      sh "git rev-parse --short HEAD"
      container('rust') {
        sh 'cargo build --release'
      }
    }
  }
}