#[doc = "Register `STCR` reader"]
pub struct R(crate::R<STCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCR` writer"]
pub struct W(crate::W<STCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCR_SPEC>;
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
impl From<crate::W<STCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BFDCL` reader - BFDF Clear"]
pub type BFDCL_R = crate::BitReader<bool>;
#[doc = "Field `BFDCL` writer - BFDF Clear"]
pub type BFDCL_W<'a, const O: u8> = crate::BitWriter<'a, u8, STCR_SPEC, bool, O>;
#[doc = "Field `CF0MCL` reader - CF0MF Clear"]
pub type CF0MCL_R = crate::BitReader<bool>;
#[doc = "Field `CF0MCL` writer - CF0MF Clear"]
pub type CF0MCL_W<'a, const O: u8> = crate::BitWriter<'a, u8, STCR_SPEC, bool, O>;
#[doc = "Field `CF1MCL` reader - CF1MF Clear"]
pub type CF1MCL_R = crate::BitReader<bool>;
#[doc = "Field `CF1MCL` writer - CF1MF Clear"]
pub type CF1MCL_W<'a, const O: u8> = crate::BitWriter<'a, u8, STCR_SPEC, bool, O>;
#[doc = "Field `PIBDCL` reader - PIBDF Clear"]
pub type PIBDCL_R = crate::BitReader<bool>;
#[doc = "Field `PIBDCL` writer - PIBDF Clear"]
pub type PIBDCL_W<'a, const O: u8> = crate::BitWriter<'a, u8, STCR_SPEC, bool, O>;
#[doc = "Field `BCDCL` reader - BCDF Clear"]
pub type BCDCL_R = crate::BitReader<bool>;
#[doc = "Field `BCDCL` writer - BCDF Clear"]
pub type BCDCL_W<'a, const O: u8> = crate::BitWriter<'a, u8, STCR_SPEC, bool, O>;
#[doc = "Field `AEDCL` reader - AEDF Clear"]
pub type AEDCL_R = crate::BitReader<bool>;
#[doc = "Field `AEDCL` writer - AEDF Clear"]
pub type AEDCL_W<'a, const O: u8> = crate::BitWriter<'a, u8, STCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BFDF Clear"]
    #[inline(always)]
    pub fn bfdcl(&self) -> BFDCL_R {
        BFDCL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CF0MF Clear"]
    #[inline(always)]
    pub fn cf0mcl(&self) -> CF0MCL_R {
        CF0MCL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CF1MF Clear"]
    #[inline(always)]
    pub fn cf1mcl(&self) -> CF1MCL_R {
        CF1MCL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PIBDF Clear"]
    #[inline(always)]
    pub fn pibdcl(&self) -> PIBDCL_R {
        PIBDCL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BCDF Clear"]
    #[inline(always)]
    pub fn bcdcl(&self) -> BCDCL_R {
        BCDCL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AEDF Clear"]
    #[inline(always)]
    pub fn aedcl(&self) -> AEDCL_R {
        AEDCL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BFDF Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bfdcl(&mut self) -> BFDCL_W<0> {
        BFDCL_W::new(self)
    }
    #[doc = "Bit 1 - CF0MF Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cf0mcl(&mut self) -> CF0MCL_W<1> {
        CF0MCL_W::new(self)
    }
    #[doc = "Bit 2 - CF1MF Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cf1mcl(&mut self) -> CF1MCL_W<2> {
        CF1MCL_W::new(self)
    }
    #[doc = "Bit 3 - PIBDF Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pibdcl(&mut self) -> PIBDCL_W<3> {
        PIBDCL_W::new(self)
    }
    #[doc = "Bit 4 - BCDF Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bcdcl(&mut self) -> BCDCL_W<4> {
        BCDCL_W::new(self)
    }
    #[doc = "Bit 5 - AEDF Clear"]
    #[inline(always)]
    #[must_use]
    pub fn aedcl(&mut self) -> AEDCL_W<5> {
        AEDCL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcr](index.html) module"]
pub struct STCR_SPEC;
impl crate::RegisterSpec for STCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [stcr::R](R) reader structure"]
impl crate::Readable for STCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stcr::W](W) writer structure"]
impl crate::Writable for STCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STCR to value 0"]
impl crate::Resettable for STCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
