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
    stage('Build') {
    checkout scm
    sh "git rev-parse --short HEAD | echo"
      container('rust') {
        sh 'cargo build --release'
      }
    }
  }
}