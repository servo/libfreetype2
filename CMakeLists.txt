project(freetype2)
cmake_minimum_required(VERSION 2.6)

# Needed to avoid error
set(DUMMY ${CMAKE_BUILD_TYPE})

function(set_prefix var prefix)
  string(REGEX REPLACE "(^|;)([^;]+)" "\\1${prefix}\\2" tmp "${ARGN}")
  set(${var} "${tmp}" PARENT_SCOPE)
endfunction()

file(WRITE ${PROJECT_BINARY_DIR}/ftmodule.h
"
/* This is a generated file. */
FT_USE_MODULE( FT_Driver_ClassRec, tt_driver_class )
FT_USE_MODULE( FT_Driver_ClassRec, t1_driver_class )
FT_USE_MODULE( FT_Driver_ClassRec, cff_driver_class )
FT_USE_MODULE( FT_Driver_ClassRec, t1cid_driver_class )
FT_USE_MODULE( FT_Driver_ClassRec, pfr_driver_class )
FT_USE_MODULE( FT_Driver_ClassRec, t42_driver_class )
FT_USE_MODULE( FT_Driver_ClassRec, winfnt_driver_class )
FT_USE_MODULE( FT_Driver_ClassRec, pcf_driver_class )
FT_USE_MODULE( FT_Driver_ClassRec, bdf_driver_class )
FT_USE_MODULE( FT_Module_Class, sfnt_module_class )
FT_USE_MODULE( FT_Module_Class, autofit_module_class )
FT_USE_MODULE( FT_Module_Class, pshinter_module_class )
FT_USE_MODULE( FT_Renderer_Class, ft_raster1_renderer_class )
FT_USE_MODULE( FT_Renderer_Class, ft_smooth_renderer_class )
FT_USE_MODULE( FT_Renderer_Class, ft_smooth_lcd_renderer_class )
FT_USE_MODULE( FT_Renderer_Class, ft_smooth_lcdv_renderer_class )
FT_USE_MODULE( FT_Module_Class, psaux_module_class )
FT_USE_MODULE( FT_Module_Class, psnames_module_class )
/* EOF */
"
)  

file(GLOB PUBLIC_H freetype2/include/freetype/*.h)
file(GLOB INTERNAL_H freetype2/include/freetype/internal/*.h)
file(GLOB SERVICES_H freetype2/include/freetype/internal/services/*.h)
file(GLOB CONFIG_H freetype2/include/freetype/config/*.h ${PROJECT_BINARY_DIR}/ftmodule.h)
file(GLOB DEVEL_H freetype2/devel/*.h)
set(FREETYPE_H ${PUBLIC_H} ${INTERNAL_H} ${SERVICES_H} ${CONFIG_H} ${DEVEL_H})
  
set_prefix(BASE_SRC freetype2/src/base/
  ftbase.h
  ftadvanc.c
  ftcalc.c
  ftdbgmem.c
  ftgloadr.c
  ftobjs.c
  ftoutln.c
  ftrfork.c
  ftsnames.c
  ftstream.c
  fttrigon.c
  ftutil.c
  ftsystem.c
  ftdebug.c
  ftinit.c
  ftbbox.c
  ftbdf.c
  ftbitmap.c
  ftcid.c
  ftfstype.c
  ftgasp.c
  ftglyph.c
  ftgxval.c
  ftlcdfil.c
  ftmm.c
  ftotval.c
  ftpatent.c
  ftpfr.c
  ftstroke.c
  ftsynth.c
  fttype1.c
  ftwinfnt.c
  ftxf86.c
  )
#truetype
set_prefix(TT_SRC freetype2/src/truetype/
  ttdriver.h
  ttdriver.c
  ttgload.h
  ttgload.c
  ttgxvar.h
  ttgxvar.c
  ttinterp.h
  ttinterp.c
  ttobjs.h
  ttobjs.c
  ttpic.h
  ttpic.c
  ttpload.h
  ttpload.c
  ttsubpix.h
  ttsubpix.c
  tterrors.h
  )
#type1
set_prefix(T1_SRC freetype2/src/type1/
  t1parse.h
  t1parse.c
  t1load.h
  t1load.c
  t1driver.h
  t1driver.c
  t1afm.h
  t1afm.c
  t1gload.h
  t1gload.c
  t1objs.h
  t1objs.c
  t1tokens.h
  t1errors.h
  )
#cff
set_prefix(CFF_SRC freetype2/src/cff/
  cffcmap.h
  cffcmap.c
  cffdrivr.h
  cffdrivr.c
  cffgload.h
  cffgload.c
  cffload.h
  cffload.c
  cffobjs.h
  cffobjs.c
  cffparse.h
  cffparse.c
  cffpic.h
  cffpic.c
  cfferrs.h
  cfftoken.h
  cfftypes.h
  )
#cid
set_prefix(CID_SRC freetype2/src/cid/
  cidparse.h
  cidparse.c
  cidload.h
  cidload.c
  cidriver.h
  cidriver.c
  cidgload.h
  cidgload.c
  cidobjs.h
  cidobjs.c
  cidtoken.h
  ciderrs.h
  )
#pfr
set_prefix(PFR_SRC freetype2/src/pfr/
  pfrload.h
  pfrload.c
  pfrgload.h
  pfrgload.c
  pfrcmap.h
  pfrcmap.c
  pfrdrivr.h
  pfrdrivr.c
  pfrsbit.h
  pfrsbit.c
  pfrobjs.h
  pfrobjs.c
  pfrerror.h
  pfrtypes.h
  )
#type42
set_prefix(T42_SRC freetype2/src/type42/
  t42objs.h
  t42objs.c
  t42parse.h
  t42parse.c
  t42drivr.h
  t42drivr.c
  t42error.h
  t42types.h
  )
#winfonts
set_prefix(FNT_SRC freetype2/src/winfonts/
  winfnt.h
  winfnt.c
  fnterrs.h
  )
#pcf
set_prefix(PCF_SRC freetype2/src/pcf/
  pcfdrivr.h
  pcfdrivr.c
  pcfread.h
  pcfread.c
  pcfutil.h
  pcfutil.c
  pcf.h
  pcferror.h
  )
#bdf
set_prefix(BDF_SRC freetype2/src/bdf/
  bdflib.c
  bdfdrivr.c
  bdf.h
  bdfdrivr.h
  bdferror.h
  )
#sfnt
set_prefix(SFNT_SRC freetype2/src/sfnt/
  ttload.h
  ttload.c
  ttmtx.h
  ttmtx.c
  ttcmap.h
  ttcmap.c
  ttsbit.h
  ttsbit.c
  ttpost.h
  ttpost.c
  ttkern.h
  ttkern.c
  ttbdf.h
  ttbdf.c
  sfobjs.h
  sfobjs.c
  sfdriver.h
  sfdriver.c
  sfntpic.h
  sfntpic.c
  sferrors.h
  )
#autofit
set_prefix(AUTOF_SRC freetype2/src/autofit/
  afangles.h
  afangles.c
  afcjk.h
  afcjk.c
  afdummy.h
  afdummy.c
  afglobal.h
  afglobal.c
  afhints.h
  afhints.c
  afindic.h
  afindic.c
  aflatin.h
  aflatin.c
  afloader.h
  afloader.c
  afmodule.h
  afmodule.c
  afpic.h
  afpic.c
  afwarp.h
  afwarp.c
  aferrors.h
  aftypes.h
  )
#pshinter
set_prefix(PSHINTER_SRC freetype2/src/pshinter/
  pshalgo.h
  pshalgo.c
  pshglob.h
  pshglob.c
  pshmod.h
  pshmod.c
  pshpic.h
  pshpic.c
  pshrec.h
  pshrec.c
  pshnterr.h
  )
#raster
set_prefix(RASTER_SRC freetype2/src/raster/
  ftraster.h
  ftraster.c
  ftrend1.h
  ftrend1.c
  rastpic.h
  rastpic.c
  rasterrs.h
  )
#smooth
set_prefix(SMOOTH_SRC freetype2/src/smooth/
  ftgrays.h
  ftgrays.c
  ftsmooth.h
  ftsmooth.c
  ftspic.h
  ftspic.c
  ftsmerrs.h
  )
#cache
set_prefix(CACHE_SRC freetype2/src/cache/
  ftcbasic.c
  ftccache.c
  ftccmap.c
  ftcglyph.c
  ftcimage.c
  ftcmanag.c
  ftcmru.c
  ftcsbits.c
  ftccache.h
  ftccback.h
  ftcerror.h
  ftcglyph.h
  ftcimage.h
  ftcmanag.h
  ftcmru.h
  ftcsbits.h
  )
#gzip
set(GZIP_SRC freetype2/src/gzip/ftgzip.c)
#lzw
set_prefix(LZW_SRC freetype2/src/lzw/
  ftlzw.c
  ftzopen.h
  )
#psaux
set_prefix(PSAUX_SRC freetype2/src/psaux/
  psobjs.c
  t1decode.c
  t1cmap.c
  afmparse.c
  psconv.c
  psauxmod.c
)
# psnames
set_prefix(PSNAMES_SRC freetype2/src/psnames/
  psmodule.c
  pspic.c
)
# ALL
set(ALL_SRC
  ${BASE_SRC}
  ${TT_SRC}
  ${T1_SRC}
  ${CFF_SRC}
  ${CID_SRC}
  ${PFR_SRC}
  ${T42_SRC}
  ${FNT_SRC}
  ${PCF_SRC}
  ${BDF_SRC}
  ${SFNT_SRC}
  ${AUTOF_SRC}
  ${PSHINTER_SRC}
  ${RASTER_SRC}
  ${SMOOTH_SRC}
  ${CACHE_SRC}
  ${GZIP_SRC}
  ${LZW_SRC}
  ${PSAUX_SRC}
  ${PSNAMES_SRC}
  ${FREETYPE_H}
  )
add_definitions(-DFT2_BUILD_LIBRARY)
include_directories(freetype2/include)
add_library(freetype STATIC ${ALL_SRC})
install(TARGETS freetype ARCHIVE DESTINATION lib)
install(DIRECTORY freetype2/include/ DESTINATION include)
