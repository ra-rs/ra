#[doc = "Register `MMPUCTL%s` reader"]
pub struct R(crate::R<MMPUCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMPUCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMPUCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMPUCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMPUCTL%s` writer"]
pub struct W(crate::W<MMPUCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMPUCTL_SPEC>;
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
impl From<crate::W<MMPUCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMPUCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Master Group enable"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "Master Group enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: Master Group is disabled. Permission of all regions."]
    _0 = 0,
    #[doc = "1: Master Group is enabled. Protection of all regions."]
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
#[doc = "Field `ENABLE` writer - Master Group enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, MMPUCTL_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "Master Group is disabled. Permission of all regions."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENABLE_A::_0)
    }
    #[doc = "Master Group is enabled. Protection of all regions."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENABLE_A::_1)
    }
}
#[doc = "Field `OAD` reader - Operation after detection"]
pub type OAD_R = crate::BitReader<OAD_A>;
#[doc = "Operation after detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OAD_A {
    #[doc = "0: Non-maskable interrupt."]
    _0 = 0,
    #[doc = "1: Internal reset."]
    _1 = 1,
}
impl From<OAD_A> for bool {
    #[inline(always)]
    fn from(variant: OAD_A) -> Self {
        variant as u8 != 0
    }
}
impl OAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OAD_A {
        match self.bits {
            false => OAD_A::_0,
            true => OAD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OAD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OAD_A::_1
    }
}
#[doc = "Field `OAD` writer - Operation after detection"]
pub type OAD_W<'a, const O: u8> = crate::BitWriter<'a, u16, MMPUCTL_SPEC, OAD_A, O>;
impl<'a, const O: u8> OAD_W<'a, O> {
    #[doc = "Non-maskable interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OAD_A::_0)
    }
    #[doc = "Internal reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OAD_A::_1)
    }
}
#[doc = "Write Keyword The data written to these bits are not stored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_AW {
    #[doc = "165: Writing to the OAD and ENABLE bit is valid, when the KEY bits are written 0xA5."]
    _0X_A5 = 165,
}
impl From<KEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` writer - Write Keyword The data written to these bits are not stored."]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, MMPUCTL_SPEC, u8, KEY_AW, 8, O>;
impl<'a, const O: u8> KEY_W<'a, O> {
    #[doc = "Writing to the OAD and ENABLE bit is valid, when the KEY bits are written 0xA5."]
    #[inline(always)]
    pub fn _0x_a5(self) -> &'a mut W {
        self.variant(KEY_AW::_0X_A5)
    }
}
impl R {
    #[doc = "Bit 0 - Master Group enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operation after detection"]
    #[inline(always)]
    pub fn oad(&self) -> OAD_R {
        OAD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Group enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Operation after detection"]
    #[inline(always)]
    #[must_use]
    pub fn oad(&mut self) -> OAD_W<1> {
        OAD_W::new(self)
    }
    #[doc = "Bits 8:15 - Write Keyword The data written to these bits are not stored."]
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
#[doc = "Bus Master MPU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmpuctl](index.html) module"]
pub struct MMPUCTL_SPEC;
impl crate::RegisterSpec for MMPUCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mmpuctl::R](R) reader structure"]
impl crate::Readable for MMPUCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmpuctl::W](W) writer structure"]
impl crate::Writable for MMPUCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUCTL%s to value 0"]
impl crate::Resettable for MMPUCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
