pipeline {
    agent any

    environment {
        PATH = "${env.HOME}/.cargo/bin:${env.PATH}"
        APP_NAME = "rust_pipeline_demo"
        NEXUS_URL = "http://localhost:8081"
        NEXUS_REPO = "rust-artifacts"
        ARTIFACT_NAME = "rust_pipeline_demo-${BUILD_NUMBER}.tar.gz"
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
                sh '''
                    rustc --version
                    cargo --version
                    git rev-parse --short HEAD
                '''
            }
        }

        stage('Build') {
            steps {
                sh 'cargo build --release'
            }
            post {
                success {
                    echo 'Build OK'
                }
                failure {
                    echo 'Build FAILED'
                }
            }
        }

        stage('Test') {
            steps {
                sh 'cargo test -- --show-output'
            }
            post {
                success {
                    echo 'Tous les tests sont passés'
                }
                failure {
                    echo 'Des tests ont échoué'
                }
            }
        }

        stage('Lint (Clippy)') {
            steps {
                sh '''
                    rustup component add clippy || true
                    cargo clippy -- -D warnings
                '''
            }
        }

        stage('Format Check') {
            steps {
                sh '''
                    rustup component add rustfmt || true
                    cargo fmt -- --check
                '''
            }
        }

        stage('Archive Artefacts') {
            steps {
                sh '''
                    test -f target/release/${APP_NAME}
                '''
                archiveArtifacts artifacts: "target/release/${APP_NAME}",
                                 fingerprint: true,
                                 allowEmptyArchive: false
            }
        }

        stage('Prepare Artifact') {
            steps {
                sh '''
                    rm -rf artifacts
                    mkdir -p artifacts
                    cp target/release/${APP_NAME} artifacts/
                    tar -czf ${ARTIFACT_NAME} artifacts
                    ls -lh ${ARTIFACT_NAME}
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
                        set -e
        
                        curl -f -u "$NEXUS_USER:$NEXUS_PASS" \
                             --upload-file "${ARTIFACT_NAME}" \
                             "${NEXUS_URL}/repository/${NEXUS_REPO}/${ARTIFACT_NAME}"
                    '''
                }
        
                echo "Artefact Nexus : ${env.NEXUS_URL}/repository/${env.NEXUS_REPO}/${env.ARTIFACT_NAME}"
                echo "Navigation Nexus : ${env.NEXUS_URL}/service/rest/repository/browse/${env.NEXUS_REPO}/"
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
