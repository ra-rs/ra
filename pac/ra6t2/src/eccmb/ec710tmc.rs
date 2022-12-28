#[doc = "Register `EC710TMC` reader"]
pub struct R(crate::R<EC710TMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EC710TMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EC710TMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EC710TMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EC710TMC` writer"]
pub struct W(crate::W<EC710TMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EC710TMC_SPEC>;
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
impl From<crate::W<EC710TMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EC710TMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECDCS` reader - ECC Decode Input Select"]
pub type ECDCS_R = crate::BitReader<ECDCS_A>;
#[doc = "ECC Decode Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECDCS_A {
    #[doc = "0: Input lower 32 bits of RAM output data to data area of decode circuit"]
    _0 = 0,
    #[doc = "1: Input ECEDB31-0 in EC710TED register to data area of decode circuit"]
    _1 = 1,
}
impl From<ECDCS_A> for bool {
    #[inline(always)]
    fn from(variant: ECDCS_A) -> Self {
        variant as u8 != 0
    }
}
impl ECDCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECDCS_A {
        match self.bits {
            false => ECDCS_A::_0,
            true => ECDCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECDCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECDCS_A::_1
    }
}
#[doc = "Field `ECDCS` writer - ECC Decode Input Select"]
pub type ECDCS_W<'a, const O: u8> = crate::BitWriter<'a, u16, EC710TMC_SPEC, ECDCS_A, O>;
impl<'a, const O: u8> ECDCS_W<'a, O> {
    #[doc = "Input lower 32 bits of RAM output data to data area of decode circuit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECDCS_A::_0)
    }
    #[doc = "Input ECEDB31-0 in EC710TED register to data area of decode circuit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECDCS_A::_1)
    }
}
#[doc = "Field `ECTMCE` reader - ECC Test Mode Control Enable"]
pub type ECTMCE_R = crate::BitReader<ECTMCE_A>;
#[doc = "ECC Test Mode Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECTMCE_A {
    #[doc = "0: The access to test mode register and bit is disabled"]
    _0 = 0,
    #[doc = "1: The access to test mode register and bit is enabled"]
    _1 = 1,
}
impl From<ECTMCE_A> for bool {
    #[inline(always)]
    fn from(variant: ECTMCE_A) -> Self {
        variant as u8 != 0
    }
}
impl ECTMCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECTMCE_A {
        match self.bits {
            false => ECTMCE_A::_0,
            true => ECTMCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECTMCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECTMCE_A::_1
    }
}
#[doc = "Field `ECTMCE` writer - ECC Test Mode Control Enable"]
pub type ECTMCE_W<'a, const O: u8> = crate::BitWriter<'a, u16, EC710TMC_SPEC, ECTMCE_A, O>;
impl<'a, const O: u8> ECTMCE_W<'a, O> {
    #[doc = "The access to test mode register and bit is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECTMCE_A::_0)
    }
    #[doc = "The access to test mode register and bit is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECTMCE_A::_1)
    }
}
#[doc = "Field `ETMA` reader - ECC Test Mode Bit Access Control"]
pub type ETMA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETMA` writer - ECC Test Mode Bit Access Control"]
pub type ETMA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, EC710TMC_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 1 - ECC Decode Input Select"]
    #[inline(always)]
    pub fn ecdcs(&self) -> ECDCS_R {
        ECDCS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - ECC Test Mode Control Enable"]
    #[inline(always)]
    pub fn ectmce(&self) -> ECTMCE_R {
        ECTMCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 14:15 - ECC Test Mode Bit Access Control"]
    #[inline(always)]
    pub fn etma(&self) -> ETMA_R {
        ETMA_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - ECC Decode Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn ecdcs(&mut self) -> ECDCS_W<1> {
        ECDCS_W::new(self)
    }
    #[doc = "Bit 7 - ECC Test Mode Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ectmce(&mut self) -> ECTMCE_W<7> {
        ECTMCE_W::new(self)
    }
    #[doc = "Bits 14:15 - ECC Test Mode Bit Access Control"]
    #[inline(always)]
    #[must_use]
    pub fn etma(&mut self) -> ETMA_W<14> {
        ETMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC Test Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ec710tmc](index.html) module"]
pub struct EC710TMC_SPEC;
impl crate::RegisterSpec for EC710TMC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ec710tmc::R](R) reader structure"]
impl crate::Readable for EC710TMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ec710tmc::W](W) writer structure"]
impl crate::Writable for EC710TMC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EC710TMC to value 0"]
impl crate::Resettable for EC710TMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
