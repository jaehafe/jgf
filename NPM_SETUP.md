# NPM Publishing Setup Guide

## 1. NPM Token 생성

1. [npmjs.com](https://www.npmjs.com)에 로그인
2. Profile → Access Tokens → Generate New Token
3. **Automation** 타입으로 생성 (Classic Token 사용)
4. 토큰 복사 (다시 볼 수 없음)

## 2. GitHub Secret 설정

1. GitHub Repository → Settings → Secrets and variables → Actions
2. **New repository secret** 클릭
3. Name: `NPM_TOKEN`
4. Secret: 위에서 복사한 NPM token 붙여넣기

## 3. 배포 프로세스

1. **버전 업데이트**:
   ```bash
   # npm/jgf/package.json의 version 수정
   # Cargo.toml의 version도 동일하게 맞춤
   ```

2. **버전 커밋**:
   ```bash
   git add npm/jgf/package.json Cargo.toml
   git commit -m "feat: version 0.1.3"
   git push origin main
   ```

3. **자동 배포**: GitHub Actions가 자동으로
   - 8개 플랫폼용 바이너리 빌드
   - npm 패키지들 생성 및 배포
   - GitHub Release 생성

## 4. 배포 확인

- Actions 탭에서 워크플로 진행 상황 확인
- [npm](https://www.npmjs.com/package/jgf)에서 패키지 확인
- 스모크 테스트로 설치/실행 확인

## 5. 사용자 설치

```bash
npm install -g jgf
jgf --version  # 0.1.3 출력되어야 함
```