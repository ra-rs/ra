#[doc = "Register `DTEVR` reader"]
pub struct R(crate::R<DTEVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTEVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTEVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTEVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTEVR` writer"]
pub struct W(crate::W<DTEVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTEVR_SPEC>;
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
impl From<crate::W<DTEVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTEVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTEV` reader - DTC Error Vector Number"]
pub type DTEV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTEVSAM` reader - DTC Error Vector Number SA Monitor"]
pub type DTEVSAM_R = crate::BitReader<DTEVSAM_A>;
#[doc = "DTC Error Vector Number SA Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEVSAM_A {
    #[doc = "0: Secure vector number"]
    _0 = 0,
    #[doc = "1: Non-Secure vector number"]
    _1 = 1,
}
impl From<DTEVSAM_A> for bool {
    #[inline(always)]
    fn from(variant: DTEVSAM_A) -> Self {
        variant as u8 != 0
    }
}
impl DTEVSAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEVSAM_A {
        match self.bits {
            false => DTEVSAM_A::_0,
            true => DTEVSAM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTEVSAM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTEVSAM_A::_1
    }
}
#[doc = "Field `DTESTA` reader - DTC Error Status Flag"]
pub type DTESTA_R = crate::BitReader<DTESTA_A>;
#[doc = "DTC Error Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTESTA_A {
    #[doc = "0: No DTC transfer error occurred"]
    _0 = 0,
    #[doc = "1: DTC transfer error occurred"]
    _1 = 1,
}
impl From<DTESTA_A> for bool {
    #[inline(always)]
    fn from(variant: DTESTA_A) -> Self {
        variant as u8 != 0
    }
}
impl DTESTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTESTA_A {
        match self.bits {
            false => DTESTA_A::_0,
            true => DTESTA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTESTA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTESTA_A::_1
    }
}
#[doc = "Field `DTESTA` writer - DTC Error Status Flag"]
pub type DTESTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTEVR_SPEC, DTESTA_A, O>;
impl<'a, const O: u8> DTESTA_W<'a, O> {
    #[doc = "No DTC transfer error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTESTA_A::_0)
    }
    #[doc = "DTC transfer error occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTESTA_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - DTC Error Vector Number"]
    #[inline(always)]
    pub fn dtev(&self) -> DTEV_R {
        DTEV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - DTC Error Vector Number SA Monitor"]
    #[inline(always)]
    pub fn dtevsam(&self) -> DTEVSAM_R {
        DTEVSAM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - DTC Error Status Flag"]
    #[inline(always)]
    pub fn dtesta(&self) -> DTESTA_R {
        DTESTA_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - DTC Error Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dtesta(&mut self) -> DTESTA_W<16> {
        DTESTA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DTC Error Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtevr](index.html) module"]
pub struct DTEVR_SPEC;
impl crate::RegisterSpec for DTEVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtevr::R](R) reader structure"]
impl crate::Readable for DTEVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtevr::W](W) writer structure"]
impl crate::Writable for DTEVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTEVR to value 0"]
impl crate::Resettable for DTEVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
