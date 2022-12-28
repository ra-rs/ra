#[doc = "Register `MMPUPTC` reader"]
pub struct R(crate::R<MMPUPTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMPUPTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMPUPTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMPUPTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMPUPTC` writer"]
pub struct W(crate::W<MMPUPTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMPUPTC_SPEC>;
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
impl From<crate::W<MMPUPTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMPUPTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROTECT` reader - Protection of register (MMPUSCn, MMPUECn and MMPUACCn)"]
pub type PROTECT_R = crate::BitReader<PROTECT_A>;
#[doc = "Protection of register (MMPUSCn, MMPUECn and MMPUACCn)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROTECT_A {
    #[doc = "0: All Bus Master MPU Group C register writing is possible."]
    _0 = 0,
    #[doc = "1: All Bus Master MPU Group C register writing is protected. Read is possible."]
    _1 = 1,
}
impl From<PROTECT_A> for bool {
    #[inline(always)]
    fn from(variant: PROTECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PROTECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROTECT_A {
        match self.bits {
            false => PROTECT_A::_0,
            true => PROTECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PROTECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PROTECT_A::_1
    }
}
#[doc = "Field `PROTECT` writer - Protection of register (MMPUSCn, MMPUECn and MMPUACCn)"]
pub type PROTECT_W<'a, const O: u8> = crate::BitWriter<'a, u16, MMPUPTC_SPEC, PROTECT_A, O>;
impl<'a, const O: u8> PROTECT_W<'a, O> {
    #[doc = "All Bus Master MPU Group C register writing is possible."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROTECT_A::_0)
    }
    #[doc = "All Bus Master MPU Group C register writing is protected. Read is possible."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROTECT_A::_1)
    }
}
#[doc = "Write Keyword The data written to these bits are not stored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_AW {
    #[doc = "165: Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
    _0X_A5 = 165,
}
impl From<KEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` writer - Write Keyword The data written to these bits are not stored."]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, MMPUPTC_SPEC, u8, KEY_AW, 8, O>;
impl<'a, const O: u8> KEY_W<'a, O> {
    #[doc = "Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
    #[inline(always)]
    pub fn _0x_a5(self) -> &'a mut W {
        self.variant(KEY_AW::_0X_A5)
    }
}
impl R {
    #[doc = "Bit 0 - Protection of register (MMPUSCn, MMPUECn and MMPUACCn)"]
    #[inline(always)]
    pub fn protect(&self) -> PROTECT_R {
        PROTECT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protection of register (MMPUSCn, MMPUECn and MMPUACCn)"]
    #[inline(always)]
    #[must_use]
    pub fn protect(&mut self) -> PROTECT_W<0> {
        PROTECT_W::new(self)
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
#[doc = "Group C protection of register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmpuptc](index.html) module"]
pub struct MMPUPTC_SPEC;
impl crate::RegisterSpec for MMPUPTC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mmpuptc::R](R) reader structure"]
impl crate::Readable for MMPUPTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmpuptc::W](W) writer structure"]
impl crate::Writable for MMPUPTC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUPTC to value 0"]
impl crate::Resettable for MMPUPTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
