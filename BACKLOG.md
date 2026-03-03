# Undiscovered - Project Backlog

## Existing Projects to Continue

### OpenClaw Rust (Phase 7+)
- **Path**: `/Users/clemenshoenig/Documents/Software-Projects/OpenClaw_Rust`
- **Status**: Phase 6 complete, ~40K LoC, 208 tests
- **Next**: Phase 7 - Agentic loop depth, multi-agent orchestration, context overflow recovery
- **Effort**: Medium - clear roadmap exists

### RuVector Experiments
- **Path**: `/Users/clemenshoenig/Documents/Software-Projects/RuVector_source_V2`
- **Status**: v2.0.3, fully complete, ~627K LoC, 79 crates
- **Ideas**: Build a demo/UI on top, visual vector space explorer, integrate with real-world data
- **Effort**: Low-Medium

### Phago Experiments
- **Path**: `/Users/clemenshoenig/Documents/Software-Projects/Phago_Project-main`
- **Status**: v1.1.0, fully complete, 15 crates, 180+ tests
- **Ideas**: Feed real scientific data, compare emergent patterns to biological systems
- **Effort**: Low-Medium

### Quantum QKD
- **Path**: `/Users/clemenshoenig/Documents/Software-Projects/Quantum`
- **Status**: Complete, research-ready
- **Ideas**: Extend to Twin-Field QKD, WebAssembly interactive explorer, publication
- **Effort**: Medium

---

## New Project Ideas - Astronomy & Space

### Interactive 3D Milky Way Navigator
- **Data**: Gaia DR3 (1.8B stars, free TAP/ADQL API)
- **Tech**: WebGL/Three.js, Rust backend
- **What**: Fly through nearby stars, color by spectral type, show clusters/streams

### Gravitational Wave Sonification & Visualization
- **Data**: LIGO/GWOSC (90+ events, free REST API + HDF5 strain data)
- **Tech**: Web Audio API, WebGL
- **What**: Hear black holes merge, visualize spiraling objects with real parameters

### Exoplanet Habitability Explorer
- **Data**: NASA Exoplanet Archive (5,700+ planets, free TAP API)
- **Tech**: Interactive visualization
- **What**: Explore habitable zones, overlay transit light curves from TESS/Kepler

---

## New Project Ideas - Physics

### Higgs Boson Re-Discovery Tool
- **Data**: CERN Open Data (real LHC collision events, free REST API)
- **Tech**: Web-based event display
- **What**: Guide users through actual Higgs analysis, plot diphoton mass histogram

### Materials Discovery Dashboard
- **Data**: Materials Project (150K+ materials, free API key)
- **Tech**: 3D crystal structure viewer, interactive filters
- **What**: Search for materials by properties, visualize crystal structures

### Standard Model Interactive Explorer
- **Data**: PDG particle data + HEPData cross-sections
- **Tech**: Interactive web app
- **What**: Click particles to see properties, decay channels, Feynman diagrams

---

## New Project Ideas - Medicine & Biology

### Drug Interaction Network Explorer
- **Data**: DrugBank + OpenFDA adverse events (free APIs)
- **Tech**: Force-directed graph, interactive
- **What**: Input medications, see interaction network, overlay real adverse event data

### Genomic Variant Clinical Impact Browser
- **Data**: ClinVar + ENSEMBL + AlphaFold (all free APIs)
- **Tech**: 3D protein viewer, variant heatmaps
- **What**: Map pathogenic variants onto protein structures

### Protein Structure Comparison Theater
- **Data**: PDB (220K structures) + AlphaFold (200M predictions)
- **Tech**: Mol* or NGL Viewer, web-based
- **What**: Compare experimental vs AI-predicted structures, link to diseases

### Global Species Migration Visualizer
- **Data**: GBIF (2.5B records) + eBird + climate data
- **Tech**: Animated globe
- **What**: Show species range shifts over decades, overlay climate change

---

## New Project Ideas - Earth Science

### Real-Time Global Earthquake Pulse
- **Data**: USGS real-time feed + IRIS seismograms (no auth needed, GeoJSON)
- **Tech**: 3D globe, live data
- **What**: Live earthquakes on globe, click for actual seismograms

### Ocean Interior Explorer
- **Data**: Argo floats (4,000 autonomous ocean profilers) + Copernicus Marine
- **Tech**: 3D volumetric visualization
- **What**: Slice through ocean temperature/salinity, animate seasonal cycles

### Climate Time Machine
- **Data**: ERA5 reanalysis (hourly global climate since 1940, free registration)
- **Tech**: Interactive per-location explorer
- **What**: How has YOUR city's climate shifted over 80 years?

---

## New Project Ideas - Humanities & Other

### Ancient World Time Machine
- **Data**: Pleiades (37K ancient places) + CDLI cuneiform (350K tablets) + Perseus texts
- **Tech**: Interactive historical map
- **What**: Travel through time 3000 BCE-500 CE, read texts, see artifacts

### Brain Gene Expression Atlas
- **Data**: Allen Brain Atlas (free REST API)
- **Tech**: 3D volumetric brain, heatmaps
- **What**: Search genes, see where they're expressed, link to diseases

### Cross-Language Syntax Explorer
- **Data**: Universal Dependencies (150+ languages, CoNLL-U format)
- **Tech**: Interactive syntax trees
- **What**: Compare sentence structure across languages

---

## Cross-Project Combinations

- Feed astronomical/genomic data into **Phago** to see what biological computing reveals
- Use **RuVector** as semantic search backend for any of the data-heavy projects
- Apply **Quantum** QKD optimization framework to other parameter optimization problems

---

## DISCOVERY-ORIENTED OPPORTUNITIES

These are datasets where genuine new findings are realistic — not just visualization,
but actual novel knowledge. Ranked by feasibility for an independent researcher.

### Tier 1 — Achievable on a Laptop in Weeks

#### Drug Repurposing via Adverse Event Mining
- **Data**: OpenFDA FAERS (~10 GB, no auth) + DrugBank + ChEMBL
- **Discovery**: Find unexpected beneficial side effects hidden in adverse event reports. If Drug X patients report fewer heart attacks than expected, that's a repurposing signal. Several FDA-approved repurposings (sildenafil, thalidomide) were discovered this way.
- **Why untapped**: Pharmacologists lack ML skills; ML people lack pharma domain knowledge. The intersection is thin.
- **Technique**: Disproportionality analysis, network pharmacology, signal detection
- **Publishable**: Yes — drug repurposing papers from FAERS mining get published routinely

#### OEIS Mathematical Sequence Relationships
- **Data**: OEIS.org (~370K integer sequences, bulk download ~100 MB)
- **Discovery**: Find unexpected connections between sequences from different mathematical domains. Cross-correlate sequences to find that a combinatorics sequence equals a number theory sequence — that's a conjecture worth proving.
- **Why untapped**: Mathematicians explore sequences individually; systematic cross-correlation hasn't been done at scale
- **Technique**: Pairwise comparison, subsequence matching, ratio analysis
- **Publishable**: Yes — new sequence identities are regularly published in journals like the Journal of Integer Sequences

#### Historical Ship Logbook Climate Reconstruction
- **Data**: ICOADS (International Comprehensive Ocean-Atmosphere Data Set) + Old Weather citizen science transcriptions (~5 GB)
- **Discovery**: Fill gaps in pre-satellite climate records using weather observations from ship logbooks (1700s-1900s). Specific ocean regions have almost no data for certain decades.
- **Why untapped**: The data exists but is messy (handwritten logs, varying instruments). Systematic regional analysis has gaps.
- **Technique**: Statistical homogenization, spatial interpolation, anomaly detection
- **Publishable**: Yes — paleoclimate reconstruction papers are high-demand

#### Paleoclimate Proxy Cross-Correlation
- **Data**: NOAA Paleoclimatology (~1 GB, free) — tree rings, ice cores, coral, speleothems, lake sediments
- **Discovery**: Synthesize multiple proxy records for a region/period to find previously unrecognized climate events. Most studies use 1-2 proxies; systematic multi-proxy synthesis for specific regions is rare.
- **Why untapped**: Different proxy communities don't always talk to each other
- **Technique**: Time series alignment, wavelet coherence, change-point detection

### Tier 2 — Needs Good Workstation or Cloud, Weeks to Months

#### ZTF/TESS Anomalous Variable Star Discovery
- **Data**: Zwicky Transient Facility (ZTF, ~1 TB light curves via IRSA) + TESS full-frame images
- **Discovery**: Find genuinely weird variable stars — objects that don't fit known categories. Tabby's Star (KIC 8462852) was found by citizen scientists in Kepler data. ZTF has billions of light curves, most never individually inspected.
- **Why untapped**: Professional astronomers focus on known classes. The anomalies fall through the cracks.
- **Technique**: Isolation forests, autoencoders on light curves, unsupervised clustering
- **Track record**: Planet Hunters (citizen science) discovered multiple exoplanets in Kepler data that automated pipelines missed
- **Publishable**: Absolutely — "weird star" papers generate massive interest

#### Satellite Archaeology (Lost Sites from Space)
- **Data**: CORONA declassified spy satellite imagery (1960s-70s, free from USGS) + Sentinel-2 multispectral (free from Copernicus) + SRTM elevation data
- **Discovery**: Find undiscovered archaeological sites. CORONA images show the landscape BEFORE modern development destroyed sites. Comparing 1960s CORONA to modern Sentinel-2 reveals what's been lost and what might remain.
- **Real discoveries**: Researchers found thousands of previously unknown sites in Syria and Iraq using CORONA imagery. Harvard's DASH project is ongoing but covers only a fraction.
- **Why untapped**: Most of the CORONA archive has never been examined by archaeologists. Vast regions (Central Asia, North Africa, Arabian Peninsula) are barely touched.
- **Technique**: Change detection, multispectral anomaly detection, DEM analysis for subtle earthworks
- **Publishable**: High impact — Journal of Archaeological Science, Antiquity

#### Microbiome Cross-Study Meta-Analysis
- **Data**: MGnify (EBI, ~100 GB) + Human Microbiome Project + curatedMetagenomicData
- **Discovery**: Pool microbiome datasets from different studies to find microbial signatures that are too subtle to detect in individual studies. Especially powerful for rare diseases where single studies have <50 patients.
- **Why untapped**: Batch effects between studies make direct comparison difficult. But modern normalization methods (ComBat-seq, MMUPHin) have largely solved this.
- **Technique**: Differential abundance analysis, network inference, random effects meta-analysis

#### Whale Song Dialect Evolution
- **Data**: Watkins Marine Mammal Sound Database (Woods Hole) + NOAA passive acoustic monitoring (~500 GB) + Macaulay Library (Cornell)
- **Discovery**: Track how whale song patterns change over years and across populations. Humpback whale songs evolve culturally — new phrases spread across ocean basins. The dynamics are poorly understood.
- **Why untapped**: Marine bioacoustics is a tiny field. Modern spectrogram analysis + ML hasn't been systematically applied to the long-term archives.
- **Technique**: Spectrogram feature extraction, sequence alignment (borrow from genomics), clustering
- **Publishable**: Marine mammal papers in Nature/Science-level journals when they reveal cultural transmission

#### Citation Network Structural Holes
- **Data**: OpenAlex (free, ~50 GB compressed) — 250M+ scholarly works with full citation graph + metadata
- **Discovery**: Find "structural holes" — pairs of research communities that cite the same foundational work but never cite each other. These represent potential breakthrough opportunities where connecting two fields could yield novel insights.
- **Why untapped**: Bibliometricians study citation patterns, but systematic structural hole detection across ALL of science hasn't been done. OpenAlex only became fully open in 2022.
- **Technique**: Community detection, betweenness centrality, bridge identification
- **Publishable**: Yes — scientometrics journals + the specific fields where gaps are found

### Tier 3 — Significant Compute + Expertise, Months

#### Non-Coding DNA Functional Elements
- **Data**: ENCODE (Encyclopedia of DNA Elements) + Roadmap Epigenomics (~10 TB)
- **Discovery**: 98% of human DNA doesn't code for proteins. ENCODE mapped regulatory elements, but most non-coding sequence is still "dark matter." Systematic ML-based function prediction for poorly characterized regions could identify new regulatory elements, especially tissue-specific enhancers.
- **Technique**: Deep learning on sequence + epigenomic features, conservation analysis
- **Publishable**: Very high impact if validated

#### Ambient Noise Seismic Tomography
- **Data**: IRIS seismic station data (~10 TB)
- **Discovery**: Use background seismic noise (ocean waves, traffic, wind) to image Earth's interior without earthquakes. Cross-correlate continuous noise between station pairs to extract Green's functions. Can reveal subsurface structures (aquifers, volcanic chambers, fault zones) invisible to traditional methods.
- **Why untapped**: The technique is proven but has only been applied to a fraction of available station pairs
- **Technique**: Cross-correlation, phase velocity measurement, tomographic inversion

#### Ancient DNA Population Genetics
- **Data**: Allen Ancient DNA Resource (Harvard, ~1 TB) — 15,000+ published ancient genomes
- **Discovery**: Reconstruct migration patterns and population mixing events in prehistory. The dataset grows rapidly as new papers publish genomes, but systematic re-analysis of the full dataset with updated methods lags behind.
- **Technique**: PCA, ADMIXTURE, f-statistics, qpAdm modeling

### Cross-Domain Discovery Opportunities (Highest Potential)

These sit at intersections no single discipline owns:

#### Climate × Disease Correlations
- **Combine**: ERA5 climate reanalysis + WHO Global Health Observatory + CDC WONDER
- **Discovery**: Find lagged correlations between specific climate patterns and disease outbreaks at fine spatiotemporal resolution. Most epidemiology uses crude annual/national climate averages.

#### Astronomical Events × Historical Records × Tree Rings
- **Combine**: Historical "guest star" records (Chinese, Korean, Arabic) + modern supernova remnant catalogs + IntCal20 tree ring radiocarbon
- **Discovery**: Match undated historical celestial events to specific astrophysical objects. The 774 CE Miyake event was only found in 2012. Similar events almost certainly exist.

#### Music × Language × Environment Co-Evolution
- **Combine**: Natural History of Song database + Glottolog language families + D-PLACE cross-cultural data
- **Discovery**: Test whether musical features co-evolve with linguistic features and environmental pressures. Nobody has systematically attempted this.

#### Infrastructure Network Vulnerability
- **Combine**: OpenStreetMap road/rail + Global Power Plant Database + AIS ship tracking
- **Discovery**: Graph-theoretic analysis to find critical single-point-of-failure infrastructure. Which one bridge, if removed, causes maximum disruption?

---

## Discovery Principles

1. **Look where others don't** — Standard algorithms on ignored datasets beat novel algorithms on popular datasets
2. **Cross-reference is king** — Combining two never-combined datasets is the fastest path to a paper
3. **Anomaly detection beats classification** — Finding what something is NOT is where discoveries live
4. **Null results matter** — Systematically finding no signal where one was expected is publishable
5. **Publish preprints** — ArXiv, bioRxiv, EarthArXiv are free, no affiliation needed

---

## Key Open Data APIs (Quick Reference)

| Dataset | Auth | Format | URL |
|---------|------|--------|-----|
| USGS Earthquakes | None | GeoJSON | earthquake.usgs.gov |
| NASA Exoplanet Archive | None | CSV/JSON | exoplanetarchive.ipac.caltech.edu |
| LIGO/GWOSC | None | JSON/HDF5 | gwosc.org |
| GBIF Biodiversity | None | JSON | gbif.org |
| OpenFDA | None | JSON | open.fda.gov |
| CERN Open Data | None | ROOT/CSV | opendata.cern.ch |
| Gaia DR3 | None | TAP/ADQL | gea.esac.esa.int |
| AlphaFold | None | PDB/JSON | alphafold.ebi.ac.uk |
| Materials Project | Free key | JSON | materialsproject.org |
| SDSS | None | CSV/JSON | skyserver.sdss.org |
| Allen Brain Atlas | None | JSON | portal.brain-map.org |
| Copernicus CDS | Free reg | NetCDF | cds.climate.copernicus.eu |
