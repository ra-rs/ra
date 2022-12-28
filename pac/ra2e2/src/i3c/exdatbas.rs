#[doc = "Register `EXDATBAS` reader"]
pub struct R(crate::R<EXDATBAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXDATBAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXDATBAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXDATBAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXDATBAS` writer"]
pub struct W(crate::W<EXDATBAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXDATBAS_SPEC>;
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
impl From<crate::W<EXDATBAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXDATBAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDSTAD` reader - Extended Device Static Address"]
pub type EDSTAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDSTAD` writer - Extended Device Static Address"]
pub type EDSTAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXDATBAS_SPEC, u8, u8, 7, O>;
#[doc = "Field `EDDYAD` reader - Extended Device I3C Dynamic Address"]
pub type EDDYAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDDYAD` writer - Extended Device I3C Dynamic Address"]
pub type EDDYAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXDATBAS_SPEC, u8, u8, 8, O>;
#[doc = "Field `EDNACK` reader - Extended Device NACK Retry Count"]
pub type EDNACK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDNACK` writer - Extended Device NACK Retry Count"]
pub type EDNACK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXDATBAS_SPEC, u8, u8, 2, O>;
#[doc = "Field `EDTYP` reader - Extended Device Type"]
pub type EDTYP_R = crate::BitReader<EDTYP_A>;
#[doc = "Extended Device Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDTYP_A {
    #[doc = "0: I3C Device"]
    _0 = 0,
    #[doc = "1: I2C Device"]
    _1 = 1,
}
impl From<EDTYP_A> for bool {
    #[inline(always)]
    fn from(variant: EDTYP_A) -> Self {
        variant as u8 != 0
    }
}
impl EDTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDTYP_A {
        match self.bits {
            false => EDTYP_A::_0,
            true => EDTYP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDTYP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDTYP_A::_1
    }
}
#[doc = "Field `EDTYP` writer - Extended Device Type"]
pub type EDTYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXDATBAS_SPEC, EDTYP_A, O>;
impl<'a, const O: u8> EDTYP_W<'a, O> {
    #[doc = "I3C Device"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDTYP_A::_0)
    }
    #[doc = "I2C Device"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDTYP_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:6 - Extended Device Static Address"]
    #[inline(always)]
    pub fn edstad(&self) -> EDSTAD_R {
        EDSTAD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - Extended Device I3C Dynamic Address"]
    #[inline(always)]
    pub fn eddyad(&self) -> EDDYAD_R {
        EDDYAD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 29:30 - Extended Device NACK Retry Count"]
    #[inline(always)]
    pub fn ednack(&self) -> EDNACK_R {
        EDNACK_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Extended Device Type"]
    #[inline(always)]
    pub fn edtyp(&self) -> EDTYP_R {
        EDTYP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Extended Device Static Address"]
    #[inline(always)]
    #[must_use]
    pub fn edstad(&mut self) -> EDSTAD_W<0> {
        EDSTAD_W::new(self)
    }
    #[doc = "Bits 16:23 - Extended Device I3C Dynamic Address"]
    #[inline(always)]
    #[must_use]
    pub fn eddyad(&mut self) -> EDDYAD_W<16> {
        EDDYAD_W::new(self)
    }
    #[doc = "Bits 29:30 - Extended Device NACK Retry Count"]
    #[inline(always)]
    #[must_use]
    pub fn ednack(&mut self) -> EDNACK_W<29> {
        EDNACK_W::new(self)
    }
    #[doc = "Bit 31 - Extended Device Type"]
    #[inline(always)]
    #[must_use]
    pub fn edtyp(&mut self) -> EDTYP_W<31> {
        EDTYP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended Device Address Table Basic Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exdatbas](index.html) module"]
pub struct EXDATBAS_SPEC;
impl crate::RegisterSpec for EXDATBAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exdatbas::R](R) reader structure"]
impl crate::Readable for EXDATBAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exdatbas::W](W) writer structure"]
impl crate::Writable for EXDATBAS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXDATBAS to value 0"]
impl crate::Resettable for EXDATBAS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
