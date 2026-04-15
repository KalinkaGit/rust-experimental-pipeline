pipeline {
    agent any

    environment {
        PATH = "${env.HOME}/.cargo/bin:${env.PATH}"
        APP_NAME = "rust_pipeline_demo"
        NEXUS_URL = "http://localhost:8081"
        NEXUS_REPO = "rust-artifacts"
        ARTIFACT_NAME = "rust_pipeline_demo-${BUILD_NUMBER}.tar.gz"
        SONAR_PROJECT_KEY = "rust_pipeline_demo"
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
                success { echo 'Build OK' }
                failure { echo 'Build FAILED' }
            }
        }

        stage('Test & Coverage') {
            steps {
                sh '''
                    cargo llvm-cov --lcov --output-path lcov.info

                    python3 -c "
lines = open('lcov.info').read().splitlines()
output = ['<coverage version=\"1\">']
current_file = None
line_hits = {}

for line in lines:
    if line.startswith('SF:'):
        current_file = line[3:]
        line_hits = {}
    elif line.startswith('DA:'):
        parts = line[3:].split(',')
        lineno = parts[0]
        hits = int(parts[1].split()[0])
        line_hits[lineno] = hits > 0
    elif line == 'end_of_record' and current_file:
        output.append('  <file path=\"' + current_file + '\">')
        for lineno, covered in sorted(line_hits.items(), key=lambda x: int(x[0])):
            output.append('    <lineToCover lineNumber=\"' + lineno + '\" covered=\"' + str(covered).lower() + '\"/>')
        output.append('  </file>')
        current_file = None

output.append('</coverage>')
open('coverage.xml', 'w').write('\\n'.join(output))
print('coverage.xml generated')
"
                '''
            }
            post {
                success { echo 'Tous les tests sont passés' }
                failure { echo 'Des tests ont échoué' }
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

        stage('SonarQube Analysis') {
            steps {
                withSonarQubeEnv('SonarQube') {
                    sh '''
                        sonar-scanner \
                          -Dsonar.projectKey=${SONAR_PROJECT_KEY} \
                          -Dsonar.projectName=${APP_NAME} \
                          -Dsonar.sources=src \
                          -Dsonar.coverageReportPaths=coverage.xml \
                          -Dsonar.exclusions=target/**
                    '''
                }
            }
        }

        stage('Quality Gate') {
            steps {
                timeout(time: 5, unit: 'MINUTES') {
                    waitForQualityGate abortPipeline: true
                }
            }
        }

        stage('Archive Artefacts') {
            steps {
                sh 'test -f target/release/${APP_NAME}'
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
    }

    post {
        always {
            echo "Pipeline terminé — statut : ${currentBuild.currentResult}"
            cleanWs()
        }
        success { echo 'Pipeline réussi !' }
        failure { echo 'Pipeline en échec.' }
    }
}
