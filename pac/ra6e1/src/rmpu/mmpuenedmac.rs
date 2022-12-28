#[doc = "Register `MMPUENEDMAC` reader"]
pub struct R(crate::R<MMPUENEDMAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMPUENEDMAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMPUENEDMAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMPUENEDMAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMPUENEDMAC` writer"]
pub struct W(crate::W<MMPUENEDMAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMPUENEDMAC_SPEC>;
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
impl From<crate::W<MMPUENEDMAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMPUENEDMAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Bus Master MPU of EDMAC enable"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "Bus Master MPU of EDMAC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: Bus Master MPU of EDMAC is disabled."]
    _0 = 0,
    #[doc = "1: Bus Master MPU of EDMAC is enabled."]
    _1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::_0,
            true => ENABLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENABLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENABLE_A::_1
    }
}
#[doc = "Field `ENABLE` writer - Bus Master MPU of EDMAC enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, MMPUENEDMAC_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "Bus Master MPU of EDMAC is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENABLE_A::_0)
    }
    #[doc = "Bus Master MPU of EDMAC is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENABLE_A::_1)
    }
}
#[doc = "Field `KEY` writer - These bits enable or disable writes to the ENABLE bit."]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, MMPUENEDMAC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Bus Master MPU of EDMAC enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Master MPU of EDMAC enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 8:15 - These bits enable or disable writes to the ENABLE bit."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<8> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMPU Enable Register for EDMAC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmpuenedmac](index.html) module"]
pub struct MMPUENEDMAC_SPEC;
impl crate::RegisterSpec for MMPUENEDMAC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mmpuenedmac::R](R) reader structure"]
impl crate::Readable for MMPUENEDMAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmpuenedmac::W](W) writer structure"]
impl crate::Writable for MMPUENEDMAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUENEDMAC to value 0"]
impl crate::Resettable for MMPUENEDMAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
