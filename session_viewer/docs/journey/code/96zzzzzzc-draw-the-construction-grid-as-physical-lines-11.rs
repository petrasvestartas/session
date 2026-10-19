{
  "scope": "Independent maintained native empty-scene hardware-grid raster at two physical viewport sizes, single sample. Native output is not an actual browser DPR measurement.",
  "createdAt": "2026-10-15T04:41:11.533Z",
  "inputHashes": {
    "src/camera.rs": "2eb0d51723129bdea1cddc3f35760217923959afe2a50dfe1fc6e129c8a3d58f",
    "src/shaders/grid.wgsl": "67daedfc89ed5b5553d2061429a4d725d1bc12b7f8e503d04bf1d709befc21fb",
    "src/engine/gpu/backdrop.rs": "afb51eaef8ce003e22ab9b4ce0e0af2c1ee786c98b98ceb22992f2a3edf4d6bc",
    "src/selftest.rs": "dec37fbc978002c1184181627769ac0c6daeec91da64b13373ef1d33bdaa08fc",
    "examples/selftest.rs": "af55062d4534e26d77925657d8daae8437772fb49a2a5e422a2e43e684a23b76",
    "Cargo.toml": "f3c4a4e33279d03a069b041b21d5b25be8c5f2c1846adbf406c693fc40234002",
    "Cargo.lock": "9b313409fa85751cba3224e7902022ee6a9420f7fdeaf69acf13c0ec62a6d9de",
    "src/shaders/background.wgsl": "ea62408626759db7cf4a3756d83dbbd5aead2621aeee028f4c54de4310ef2aa6",
    "src/engine/gpu/frame.rs": "2a492fe36f15db4201a687a1d06e120945d38410a075375fece6ec1d3d4323fb",
    "src/engine/gpu/render.rs": "48150fc2c3afa9a453af5fa0d0aa35768ed42ee57454a3c46ef0016a518dfabc"
  },
  "pose": {
    "targetSceneUnits": [
      0,
      0,
      0
    ],
    "eyeSceneUnits": [
      0,
      0,
      12000
    ],
    "distanceSceneUnits": 12000,
    "unit": "millimetres",
    "projection": "orthographic",
    "halfHeightSceneUnits": 6928.203230276,
    "backgroundReadback": [
      255,
      255,
      255
    ],
    "note": "The maintained background pass overwrites selftest CLEAR 0.9 with white when ambient shading is off. Match the actual white readback, not the earlier clear value."
  },
  "oracleManifestSha256": "13e832ae0d5a0153ebb5f85e72e276b23347ace053b6d799030d44b2ae8d6a72",
  "frames": [
    {
      "asset": "docs/journey/checks/data/current-grid-physical-256-2026-10-15.ppm",
      "physicalSize": [
        256,
        256
      ],
      "sha256": "a1dea16ee3639ebebc9225a529366cda93d167266b93a95d29dd6b8beca84507"
    },
    {
      "asset": "docs/journey/checks/data/current-grid-physical-512-2026-10-15.ppm",
      "physicalSize": [
        512,
        512
      ],
      "sha256": "d871ce0ed9583adc0100f2134e2ce86f7754ebf5883dc06721364edf652dda19"
    }
  ]
}
