pipeline {
    agent any

    environment {
        PATH = "${env.HOME}/.cargo/bin:${env.PATH}"
        NEXUS_URL = "http://localhost:8081"
        NEXUS_REPO = "rust-artifacts"
    }

    options {
        timestamps()
        timeout(time: 30, unit: 'MINUTES')
        buildDiscarder(logRotator(numToKeepStr: '10'))
    }

    stages {

        stage('Checkout') {
            steps {
                checkout scm
                sh 'rustc --version && cargo --version'
            }
        }

        stage('Build') {
            steps {
                sh 'cargo build --release 2>&1'
            }
            post {
                success { echo 'Build OK' }
                failure { echo 'Build FAILED' }
            }
        }

        stage('Test') {
            steps {
                sh 'cargo test -- --show-output 2>&1'
            }
            post {
                success { echo 'Tous les tests sont passés' }
                failure { echo 'Des tests ont échoué' }
            }
        }

        stage('Lint (Clippy)') {
            steps {
                sh 'rustup component add clippy || true'
                sh 'cargo clippy -- -D warnings 2>&1'
            }
        }

        stage('Format Check') {
            steps {
                sh 'rustup component add rustfmt || true'
                sh 'cargo fmt -- --check 2>&1'
            }
        }

        stage('Archive Artefacts') {
            steps {
                archiveArtifacts artifacts: 'target/release/rust_pipeline_demo',
                                 fingerprint: true,
                                 allowEmptyArchive: false
            }
        }

        stage('Prepare Artifact') {
            steps {
                sh '''
                mkdir -p artifacts
                cp target/release/rust_pipeline_demo artifacts/
                tar -czf rust_pipeline_demo.tar.gz artifacts
                '''
            }
        }

        stage('Upload to Nexus') {
            steps {
                withCredentials([usernamePassword(
                    credentialsId: 'nexus',
                    usernameVariable: 'NEXUS_USER',
                    passwordVariable: 'NEXUS_PASS'
                )]) {

                    sh '''
                    curl -u $NEXUS_USER:$NEXUS_PASS \
                         --upload-file rust_pipeline_demo.tar.gz \
                         $NEXUS_URL/repository/$NEXUS_REPO/rust_pipeline_demo-${BUILD_NUMBER}.tar.gz
                    '''
                }
            }
        }
    }

    post {
        always {
            echo "Pipeline terminé — statut : ${currentBuild.currentResult}"
            cleanWs()
        }
        success {
            echo 'Pipeline réussi !'
        }
        failure {
            echo 'Pipeline en échec.'
        }
    }
}
