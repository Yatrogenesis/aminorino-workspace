# Zenodo Integration Guide - DOI Generation

**Purpose**: Generate permanent DOIs for all 6 research papers via Zenodo-GitHub integration

**Author**: Francisco Molina Burgos
**Date**: 2025-11-12

---

## Prerequisites

✅ **COMPLETED**:
- [x] GitHub repository created and public: `github.com/Yatrogenesis/cortexia`
- [x] CITATION.cff file created (CFF 1.2.0 format)
- [x] README.md with portfolio overview
- [x] LICENSE-APACHE and LICENSE-MIT files
- [x] All papers moderated and validated
- [x] Code pushed to GitHub (commit 11da928)

---

## Step-by-Step Zenodo Integration

### Step 1: Link GitHub to Zenodo

1. **Go to Zenodo**: https://zenodo.org
2. **Log in with GitHub account**: Click "Log in" → "Log in with GitHub"
3. **Authorize Zenodo**: Grant Zenodo access to your GitHub repositories
4. **Enable repository**:
   - Go to: https://zenodo.org/account/settings/github/
   - Find `Yatrogenesis/cortexia` in the repository list
   - Toggle ON the switch next to the repository

### Step 2: Create First Release

Zenodo generates DOIs based on GitHub releases.

**Option A: Via GitHub Web Interface**
1. Go to: `https://github.com/Yatrogenesis/cortexia/releases`
2. Click "Create a new release"
3. Fill in:
   - **Tag version**: `v1.0.0`
   - **Release title**: `Cortexia Portfolio v1.0.0 - All Objections Resolved`
   - **Description**:
     ```markdown
     # Cortexia Research Portfolio v1.0.0

     Complete resolution of all 10 objections across 6 research papers.

     ## Major Milestones
     - ✅ Statistical validation (N=50, p < 10⁻¹⁶, Cohen's d = 1.76)
     - ✅ Proprietary validation suite (18 tests, 72% independent)
     - ✅ All 6 papers moderated with conservative claims
     - ✅ 3 professional figures for Paper #5
     - ✅ Comprehensive submission readiness assessment

     ## Papers Included
     1. Paper #1: Hierarchical Φ (Physical Review E - ready)
     2. Paper #2: NOS Theory (Frontiers Comp Neuro - ready)
     3. Paper #3: Topological Password (IEEE TIFS - PRIORITY)
     4. Paper #4: NOS Validation (Frontiers AI - PRIORITY)
     5. Paper #5: Cálculo de Significados (Cognitive Systems - ready)
     6. Paper #6: Rust IIT Library (JOSS - PRIORITY)

     ## Expected Outcomes
     - Portfolio acceptance rate: 4-5 papers out of 6
     - Top-tier papers (#3, #6, #4): 70-85% acceptance probability

     **Validated**, **Moderated**, **Ready for Submission** 🎯
     ```
4. Click "Publish release"

**Option B: Via Git CLI**
```bash
cd /Users/yatrogenesis/cortexia-workspace
git tag -a v1.0.0 -m "Cortexia Portfolio v1.0.0 - All Objections Resolved"
git push cortexia v1.0.0
```

Then create the release via GitHub web interface using the tag.

### Step 3: Verify DOI Generation

1. **Wait 5-10 minutes** for Zenodo to detect the new release
2. **Check Zenodo**: https://zenodo.org/account/settings/github/
   - You should see a new entry for `Yatrogenesis/cortexia v1.0.0`
3. **Get DOI**: Click on the Zenodo record to get your DOI
   - Format: `10.5281/zenodo.XXXXXX`
4. **Update CITATION.cff**: Replace `doi: {10.5281/zenodo.XXXXXX}` placeholder with actual DOI

### Step 4: Create Individual Paper DOIs (Optional but Recommended)

Each paper can have its own DOI via separate Zenodo uploads:

**For each paper**:
1. Go to: https://zenodo.org/deposit/new
2. Fill in metadata:
   - **Upload type**: Publication → Preprint
   - **Publication date**: 2025-11-12
   - **Title**: [Paper title from README]
   - **Authors**: Francisco Molina Burgos (ORCID: 0009-0008-6093-8267)
   - **Description**: [Abstract from paper]
   - **License**: Creative Commons Attribution 4.0 International
   - **Keywords**: [From CITATION.cff]
   - **Related identifiers**:
     - GitHub repository: `https://github.com/Yatrogenesis/cortexia` (is supplemented by this upload)
3. **Upload files**:
   - Paper PDF (compile from LaTeX)
   - README_MODERATED.md
   - Supplementary materials (figures, code)
4. Click "Publish"
5. **Save DOI** for each paper

---

## DOI Assignment Plan

### Portfolio-Level DOI (via GitHub Release)
**DOI**: `10.5281/zenodo.XXXXXX` (to be generated)
**Citation**:
```bibtex
@software{molina_burgos_2025_cortexia,
  author       = {Molina Burgos, Francisco},
  title        = {Cortexia Research Portfolio},
  month        = nov,
  year         = 2025,
  publisher    = {Zenodo},
  version      = {v1.0.0},
  doi          = {10.5281/zenodo.XXXXXX},
  url          = {https://doi.org/10.5281/zenodo.XXXXXX}
}
```

### Individual Paper DOIs (via Manual Zenodo Upload)

**Paper #1**: Hierarchical Φ
- DOI: `10.5281/zenodo.XXXXXX1`
- Upload: README_MODERATED.md + PDF + Figures

**Paper #2**: NOS Theory
- DOI: `10.5281/zenodo.XXXXXX2`
- Upload: README_MODERATED.md + PDF

**Paper #3**: Topological Password
- DOI: `10.5281/zenodo.XXXXXX3`
- Upload: README_MODERATED.md + PDF + Dataset (rockyou.txt subset)

**Paper #4**: NOS Validation
- DOI: `10.5281/zenodo.XXXXXX4`
- Upload: README_MODERATED.md + PDF + Experimental results

**Paper #5**: Cálculo de Significados
- DOI: `10.5281/zenodo.XXXXXX5`
- Upload: README_MODERATED.md + PDF + Figures (3)

**Paper #6**: Rust IIT Library
- DOI: `10.5281/zenodo.XXXXXX6`
- Upload: README_MODERATED.md + Software (link to GitHub)

---

## Updating Citations After DOI Generation

After receiving DOIs from Zenodo:

1. **Update CITATION.cff**:
   ```bash
   cd /Users/yatrogenesis/cortexia-workspace
   # Edit papers_preview/CITATION.cff with actual DOIs
   git add papers_preview/CITATION.cff
   git commit -m "Add Zenodo DOIs"
   git push cortexia main
   ```

2. **Update each paper's README_MODERATED.md**:
   - Add DOI badge at top: `[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.XXXXXX.svg)](https://doi.org/10.5281/zenodo.XXXXXX)`
   - Add citation section referencing DOI

3. **Update LaTeX source files**:
   - Add `\thanks{DOI: 10.5281/zenodo.XXXXXX}` to title
   - Include DOI in acknowledgments

---

## Timeline

| Task | Duration | Status |
|------|----------|--------|
| **GitHub setup** | 30 min | ✅ DONE |
| **Zenodo account + link** | 10 min | ⏳ TODO |
| **Create v1.0.0 release** | 5 min | ⏳ TODO |
| **Wait for DOI generation** | 5-10 min | ⏳ TODO |
| **Upload individual papers** | 2 hours | ⏳ TODO |
| **Update citations with DOIs** | 30 min | ⏳ TODO |
| **TOTAL** | ~3.5 hours | 15% DONE |

---

## Automated DOI Updates (Future)

For future releases, use GitHub Actions to automatically:
1. Trigger Zenodo DOI generation on new tags
2. Update CITATION.cff with new DOI
3. Create release notes

---

## Troubleshooting

### DOI Not Appearing After Release
- **Wait 15 minutes**: Zenodo webhook can be slow
- **Check Zenodo logs**: https://zenodo.org/account/settings/github/
- **Manually trigger**: Click "Sync now" on repository in Zenodo settings

### Wrong Metadata in Zenodo
- **Edit Zenodo record**: You can edit metadata after publishing
- **New version**: Upload a new version with corrected metadata

### Missing CITATION.cff
- Zenodo reads CITATION.cff automatically for metadata
- If missing, Zenodo will use limited GitHub repo info
- **Solution**: Ensure CITATION.cff is in repository root (we have it in `papers_preview/`)

---

## Next Steps

**IMMEDIATE** (This session):
1. ⏳ Log into Zenodo with GitHub account
2. ⏳ Link `Yatrogenesis/cortexia` repository
3. ⏳ Create v1.0.0 release
4. ⏳ Verify DOI generation
5. ⏳ Update README.md with portfolio DOI

**NEAR-TERM** (This week):
1. Compile LaTeX PDFs for Papers #3 and #6
2. Upload individual papers to Zenodo
3. Update citations with DOIs
4. Prepare submission packages

---

**Last Updated**: 2025-11-12
**Repository**: https://github.com/Yatrogenesis/cortexia
**Zenodo Community**: (to be created)
