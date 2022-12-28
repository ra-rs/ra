#[doc = "Register `SDATBAS0` reader"]
pub struct R(crate::R<SDATBAS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDATBAS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDATBAS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDATBAS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDATBAS0` writer"]
pub struct W(crate::W<SDATBAS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDATBAS0_SPEC>;
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
impl From<crate::W<SDATBAS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDATBAS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDSTAD` reader - Slave Device Static Address"]
pub type SDSTAD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SDSTAD` writer - Slave Device Static Address"]
pub type SDSTAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDATBAS0_SPEC, u16, u16, 10, O>;
#[doc = "Field `SDADLS` reader - Slave Device Address Length Selection"]
pub type SDADLS_R = crate::BitReader<SDADLS_A>;
#[doc = "Slave Device Address Length Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDADLS_A {
    #[doc = "0: Slave device address length 7 bits selected."]
    _0 = 0,
    #[doc = "1: Slave device address length 10 bits selected. (I2C device only)"]
    _1 = 1,
}
impl From<SDADLS_A> for bool {
    #[inline(always)]
    fn from(variant: SDADLS_A) -> Self {
        variant as u8 != 0
    }
}
impl SDADLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDADLS_A {
        match self.bits {
            false => SDADLS_A::_0,
            true => SDADLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDADLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDADLS_A::_1
    }
}
#[doc = "Field `SDADLS` writer - Slave Device Address Length Selection"]
pub type SDADLS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDATBAS0_SPEC, SDADLS_A, O>;
impl<'a, const O: u8> SDADLS_W<'a, O> {
    #[doc = "Slave device address length 7 bits selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDADLS_A::_0)
    }
    #[doc = "Slave device address length 10 bits selected. (I2C device only)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDADLS_A::_1)
    }
}
#[doc = "Field `SDIBIPL` reader - Slave Device IBI Payload"]
pub type SDIBIPL_R = crate::BitReader<SDIBIPL_A>;
#[doc = "Slave Device IBI Payload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIBIPL_A {
    #[doc = "0: IBIs from this device do not carry a data payload."]
    _0 = 0,
    #[doc = "1: IBIs from this device carry a data payload."]
    _1 = 1,
}
impl From<SDIBIPL_A> for bool {
    #[inline(always)]
    fn from(variant: SDIBIPL_A) -> Self {
        variant as u8 != 0
    }
}
impl SDIBIPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIBIPL_A {
        match self.bits {
            false => SDIBIPL_A::_0,
            true => SDIBIPL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDIBIPL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDIBIPL_A::_1
    }
}
#[doc = "Field `SDIBIPL` writer - Slave Device IBI Payload"]
pub type SDIBIPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDATBAS0_SPEC, SDIBIPL_A, O>;
impl<'a, const O: u8> SDIBIPL_W<'a, O> {
    #[doc = "IBIs from this device do not carry a data payload."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDIBIPL_A::_0)
    }
    #[doc = "IBIs from this device carry a data payload."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDIBIPL_A::_1)
    }
}
#[doc = "Field `SDDYAD` reader - Slave Device I3C Dynamic Address"]
pub type SDDYAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDDYAD` writer - Slave Device I3C Dynamic Address"]
pub type SDDYAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDATBAS0_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:9 - Slave Device Static Address"]
    #[inline(always)]
    pub fn sdstad(&self) -> SDSTAD_R {
        SDSTAD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Slave Device Address Length Selection"]
    #[inline(always)]
    pub fn sdadls(&self) -> SDADLS_R {
        SDADLS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Slave Device IBI Payload"]
    #[inline(always)]
    pub fn sdibipl(&self) -> SDIBIPL_R {
        SDIBIPL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Slave Device I3C Dynamic Address"]
    #[inline(always)]
    pub fn sddyad(&self) -> SDDYAD_R {
        SDDYAD_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave Device Static Address"]
    #[inline(always)]
    #[must_use]
    pub fn sdstad(&mut self) -> SDSTAD_W<0> {
        SDSTAD_W::new(self)
    }
    #[doc = "Bit 10 - Slave Device Address Length Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sdadls(&mut self) -> SDADLS_W<10> {
        SDADLS_W::new(self)
    }
    #[doc = "Bit 12 - Slave Device IBI Payload"]
    #[inline(always)]
    #[must_use]
    pub fn sdibipl(&mut self) -> SDIBIPL_W<12> {
        SDIBIPL_W::new(self)
    }
    #[doc = "Bits 16:22 - Slave Device I3C Dynamic Address"]
    #[inline(always)]
    #[must_use]
    pub fn sddyad(&mut self) -> SDDYAD_W<16> {
        SDDYAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Device Address Table Basic Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdatbas0](index.html) module"]
pub struct SDATBAS0_SPEC;
impl crate::RegisterSpec for SDATBAS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdatbas0::R](R) reader structure"]
impl crate::Readable for SDATBAS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdatbas0::W](W) writer structure"]
impl crate::Writable for SDATBAS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDATBAS0 to value 0"]
impl crate::Resettable for SDATBAS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
