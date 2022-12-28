#[doc = "Register `CFDTMDF%s_14` reader"]
pub struct R(crate::R<CFDTMDF_14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDTMDF_14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDTMDF_14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDTMDF_14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDTMDF%s_14` writer"]
pub struct W(crate::W<CFDTMDF_14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDTMDF_14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFDTMDF_14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDTMDF_14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMDB_LL` reader - TX Message Buffer Data Byte ((p Ã\u{97} 4)"]
pub type TMDB_LL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMDB_LL` writer - TX Message Buffer Data Byte ((p Ã\u{97} 4)"]
pub type TMDB_LL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDTMDF_14_SPEC, u8, u8, 8, O>;
#[doc = "Field `TMDB_LH` reader - TX Message Buffer Data Byte ((p Ã\u{97} 4) + 1)"]
pub type TMDB_LH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMDB_LH` writer - TX Message Buffer Data Byte ((p Ã\u{97} 4) + 1)"]
pub type TMDB_LH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDTMDF_14_SPEC, u8, u8, 8, O>;
#[doc = "Field `TMDB_HL` reader - TX Message Buffer Data Byte ((p Ã\u{97} 4) + 2)"]
pub type TMDB_HL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMDB_HL` writer - TX Message Buffer Data Byte ((p Ã\u{97} 4) + 2)"]
pub type TMDB_HL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDTMDF_14_SPEC, u8, u8, 8, O>;
#[doc = "Field `TMDB_HH` reader - TX Message Buffer Data Byte ((p Ã\u{97} 4) + 3)"]
pub type TMDB_HH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMDB_HH` writer - TX Message Buffer Data Byte ((p Ã\u{97} 4) + 3)"]
pub type TMDB_HH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDTMDF_14_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - TX Message Buffer Data Byte ((p Ã\u{97} 4)"]
    #[inline(always)]
    pub fn tmdb_ll(&self) -> TMDB_LL_R {
        TMDB_LL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TX Message Buffer Data Byte ((p Ã\u{97} 4) + 1)"]
    #[inline(always)]
    pub fn tmdb_lh(&self) -> TMDB_LH_R {
        TMDB_LH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TX Message Buffer Data Byte ((p Ã\u{97} 4) + 2)"]
    #[inline(always)]
    pub fn tmdb_hl(&self) -> TMDB_HL_R {
        TMDB_HL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - TX Message Buffer Data Byte ((p Ã\u{97} 4) + 3)"]
    #[inline(always)]
    pub fn tmdb_hh(&self) -> TMDB_HH_R {
        TMDB_HH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TX Message Buffer Data Byte ((p Ã\u{97} 4)"]
    #[inline(always)]
    #[must_use]
    pub fn tmdb_ll(&mut self) -> TMDB_LL_W<0> {
        TMDB_LL_W::new(self)
    }
    #[doc = "Bits 8:15 - TX Message Buffer Data Byte ((p Ã\u{97} 4) + 1)"]
    #[inline(always)]
    #[must_use]
    pub fn tmdb_lh(&mut self) -> TMDB_LH_W<8> {
        TMDB_LH_W::new(self)
    }
    #[doc = "Bits 16:23 - TX Message Buffer Data Byte ((p Ã\u{97} 4) + 2)"]
    #[inline(always)]
    #[must_use]
    pub fn tmdb_hl(&mut self) -> TMDB_HL_W<16> {
        TMDB_HL_W::new(self)
    }
    #[doc = "Bits 24:31 - TX Message Buffer Data Byte ((p Ã\u{97} 4) + 3)"]
    #[inline(always)]
    #[must_use]
    pub fn tmdb_hh(&mut self) -> TMDB_HH_W<24> {
        TMDB_HH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Message Buffer Data Field Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdtmdf_14](index.html) module"]
pub struct CFDTMDF_14_SPEC;
impl crate::RegisterSpec for CFDTMDF_14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdtmdf_14::R](R) reader structure"]
impl crate::Readable for CFDTMDF_14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdtmdf_14::W](W) writer structure"]
impl crate::Writable for CFDTMDF_14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDTMDF%s_14 to value 0"]
impl crate::Resettable for CFDTMDF_14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
