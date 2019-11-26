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
    env:
    - name: CONTAINER_ENV_VAR
      value: container-env-var-value
"""
  ) {

  node(POD_LABEL) {
    stage('Build') {
      container('rust') {
          sh 'cargo build --release'
      }
    }
  }
}