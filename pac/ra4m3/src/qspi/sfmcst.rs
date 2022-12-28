#[doc = "Register `SFMCST` reader"]
pub struct R(crate::R<SFMCST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFMCST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFMCST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFMCST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFMCST` writer"]
pub struct W(crate::W<SFMCST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFMCST_SPEC>;
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
impl From<crate::W<SFMCST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFMCST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMBSY` reader - SPI bus cycle completion state in direct communication"]
pub type COMBSY_R = crate::BitReader<COMBSY_A>;
#[doc = "SPI bus cycle completion state in direct communication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMBSY_A {
    #[doc = "0: No serial transfer being processed"]
    _0 = 0,
    #[doc = "1: Serial transfer being processed"]
    _1 = 1,
}
impl From<COMBSY_A> for bool {
    #[inline(always)]
    fn from(variant: COMBSY_A) -> Self {
        variant as u8 != 0
    }
}
impl COMBSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMBSY_A {
        match self.bits {
            false => COMBSY_A::_0,
            true => COMBSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COMBSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COMBSY_A::_1
    }
}
#[doc = "Field `EROMR` reader - ROM access detection status in direct communication mode"]
pub type EROMR_R = crate::BitReader<EROMR_A>;
#[doc = "ROM access detection status in direct communication mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EROMR_A {
    #[doc = "0: ROM access not detected"]
    _0 = 0,
    #[doc = "1: ROM access detected"]
    _1 = 1,
}
impl From<EROMR_A> for bool {
    #[inline(always)]
    fn from(variant: EROMR_A) -> Self {
        variant as u8 != 0
    }
}
impl EROMR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EROMR_A {
        match self.bits {
            false => EROMR_A::_0,
            true => EROMR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EROMR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EROMR_A::_1
    }
}
#[doc = "Field `EROMR` writer - ROM access detection status in direct communication mode"]
pub type EROMR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMCST_SPEC, EROMR_A, O>;
impl<'a, const O: u8> EROMR_W<'a, O> {
    #[doc = "ROM access not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EROMR_A::_0)
    }
    #[doc = "ROM access detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EROMR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - SPI bus cycle completion state in direct communication"]
    #[inline(always)]
    pub fn combsy(&self) -> COMBSY_R {
        COMBSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - ROM access detection status in direct communication mode"]
    #[inline(always)]
    pub fn eromr(&self) -> EROMR_R {
        EROMR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - ROM access detection status in direct communication mode"]
    #[inline(always)]
    #[must_use]
    pub fn eromr(&mut self) -> EROMR_W<7> {
        EROMR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Communication Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfmcst](index.html) module"]
pub struct SFMCST_SPEC;
impl crate::RegisterSpec for SFMCST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfmcst::R](R) reader structure"]
impl crate::Readable for SFMCST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfmcst::W](W) writer structure"]
impl crate::Writable for SFMCST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFMCST to value 0"]
impl crate::Resettable for SFMCST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
