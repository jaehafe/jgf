# JGF (Jira Git Flow) 🚀

> Jira와 Git을 연동하는 워크플로우 자동화 CLI 도구

## 📌 문제 상황

개발팀에서 반복적으로 겪는 문제들:
- PM이 Jira 티켓 할당 → 개발자가 수동으로 티켓 확인
- 티켓 번호로 브랜치 생성 → 수동으로 Jira 상태 변경
- PR 생성 시 Jira 링크 복사/붙여넣기
- PR 머지 후 Jira 상태 수동 업데이트
- 로컬 브랜치 정리

**이 모든 과정이 시간이 많이 소요됩니다.**

## ✨ JGF가 해결하는 것

1. **자동 브랜치 생성**: Jira 티켓 번호로 자동 브랜치 생성
2. **자동 상태 동기화**: 작업 시작/PR/머지 시 Jira 상태 자동 업데이트
3. **PR 템플릿 자동화**: Jira 링크와 티켓 정보 자동 포함
4. **브랜치 자동 정리**: 머지된 브랜치 자동 감지 및 삭제

## 🛠 설치 방법

### npm으로 설치 (권장)

```bash
# 전역 설치
npm install -g jgf

# 또는 npx로 사용
npx jgf --version
```

### 다른 설치 방법

**Cargo (Rust 필요):**
```bash
cargo install jgf
```

**바이너리 직접 다운로드:**
- [Releases 페이지](https://github.com/jaehafe/jgf/releases)에서 플랫폼별 바이너리 다운로드

## 🔄 워크플로우

### 전체 플로우
```
1. PM/개발자 Jira 티켓 할당
    ↓
2. 개발자: jgf tickets (티켓 조회)
    ↓
3. 개발자: jgf start EM-XXX (브랜치 생성)
    → 자동: Git 브랜치 생성
    → 자동: Jira 상태 "In Progress"로 변경
    ↓
4. 개발자: 코딩 작업
    ↓
5. 개발자: jgf pr (PR 생성)
    → 자동: PR 제목에 티켓 번호 포함
    → 자동: PR 본문에 Jira 링크 추가
    ↓
6. 팀: 코드 리뷰 & 머지
    ↓
7. 개발자: jgf sync (동기화)
    → 자동: 머지된 브랜치 감지
    → 자동: Jira 상태 "Done"으로 변경
    → 자동: 로컬 브랜치 삭제
```

## ⚙️ 초기 설정

### 1. 프로젝트별 설정

각 프로젝트 루트에서 실행:

```bash
jgf init
```

### 2. API 토큰 발급

**Jira API Token:**
1. [Atlassian Account Settings](https://id.atlassian.com/manage-profile/security/api-tokens) 접속
2. "Create API token" 클릭
3. `.env` 파일에 저장

**GitHub Personal Access Token:**
1. GitHub Settings → Developer settings → Personal access tokens
2. "Generate new token (classic)" 클릭
3. 권한 선택: `repo` (전체)

## 📚 주요 명령어

### 티켓 조회 및 작업 시작
```bash
# 할당된 모든 티켓 조회
jgf tickets

# 특정 티켓으로 작업 시작
jgf start EM-100
```

### PR 생성
```bash
jgf pr
```

### 머지 후 동기화
```bash
jgf sync
```

## 🎯 실제 사용 예시

```bash
# 1. 할당된 티켓 확인
$ jgf tickets
🎫 [1] EM-120 사용자 프로필 기능 추가
   상태: To Do | 담당자: 김개발 | 우선순위: High

# 2. 작업 시작
$ jgf start EM-120
🚀 티켓 EM-120 작업을 시작합니다
🌿 브랜치 'EM-120'가 생성되고 체크아웃되었습니다
✅ 티켓 상태가 'In Progress'로 변경되었습니다

# 3. 코딩 작업 후 PR 생성
$ jgf pr
🚀 브랜치 'EM-120'에서 'develop'으로 PR 생성
✅ PR이 성공적으로 생성되었습니다! #123

# 4. 리뷰 & 머지 후 동기화
$ jgf sync
🔄 머지된 브랜치 동기화 시작
✅ 티켓 EM-120 상태가 'Done'으로 변경되었습니다
✅ 브랜치 'EM-120'가 삭제되었습니다
```

## 🏢 기능

- ✅ Jira 티켓 상태 자동 업데이트
- ✅ GitHub PR 자동 생성 및 템플릿 적용
- ✅ 브랜치 자동 생성 및 정리
- ✅ 여러 프로젝트 관리
- ✅ 크로스 플랫폼 지원 (Windows, macOS, Linux)

## 📖 전체 문서

자세한 설정 방법과 고급 기능은 [GitHub Repository](https://github.com/jaehafe/jgf)를 확인하세요.

## 🔗 링크

- **Repository**: [https://github.com/jaehafe/jgf](https://github.com/jaehafe/jgf)
- **Issues**: [https://github.com/jaehafe/jgf/issues](https://github.com/jaehafe/jgf/issues)
- **npm**: [https://www.npmjs.com/package/jgf](https://www.npmjs.com/package/jgf)

## 📄 License

MIT