#[doc = "Register `DVSTCTR0` reader"]
pub struct R(crate::R<DVSTCTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVSTCTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVSTCTR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVSTCTR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DVSTCTR0` writer"]
pub struct W(crate::W<DVSTCTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVSTCTR0_SPEC>;
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
impl From<crate::W<DVSTCTR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVSTCTR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RHST` reader - USB Bus Reset Status"]
pub type RHST_R = crate::FieldReader<u8, RHST_A>;
#[doc = "USB Bus Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RHST_A {
    #[doc = "0: Communication speed not determined"]
    _000 = 0,
    #[doc = "1: USB bus reset in progress or low-speed connection"]
    _001 = 1,
    #[doc = "2: USB bus reset in progress or full-speed connection"]
    _010 = 2,
}
impl From<RHST_A> for u8 {
    #[inline(always)]
    fn from(variant: RHST_A) -> Self {
        variant as _
    }
}
impl RHST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RHST_A> {
        match self.bits {
            0 => Some(RHST_A::_000),
            1 => Some(RHST_A::_001),
            2 => Some(RHST_A::_010),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RHST_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RHST_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RHST_A::_010
    }
}
#[doc = "Field `WKUP` reader - Wakeup Detection Enable"]
pub type WKUP_R = crate::BitReader<WKUP_A>;
#[doc = "Wakeup Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUP_A {
    #[doc = "0: Remote wakeup signal is not output."]
    _0 = 0,
    #[doc = "1: Remote wakeup signal is output."]
    _1 = 1,
}
impl From<WKUP_A> for bool {
    #[inline(always)]
    fn from(variant: WKUP_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUP_A {
        match self.bits {
            false => WKUP_A::_0,
            true => WKUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WKUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WKUP_A::_1
    }
}
#[doc = "Field `WKUP` writer - Wakeup Detection Enable"]
pub type WKUP_W<'a, const O: u8> = crate::BitWriter<'a, u16, DVSTCTR0_SPEC, WKUP_A, O>;
impl<'a, const O: u8> WKUP_W<'a, O> {
    #[doc = "Remote wakeup signal is not output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKUP_A::_0)
    }
    #[doc = "Remote wakeup signal is output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WKUP_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - USB Bus Reset Status"]
    #[inline(always)]
    pub fn rhst(&self) -> RHST_R {
        RHST_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Wakeup Detection Enable"]
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Wakeup Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wkup(&mut self) -> WKUP_W<8> {
        WKUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device State Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvstctr0](index.html) module"]
pub struct DVSTCTR0_SPEC;
impl crate::RegisterSpec for DVSTCTR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dvstctr0::R](R) reader structure"]
impl crate::Readable for DVSTCTR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvstctr0::W](W) writer structure"]
impl crate::Writable for DVSTCTR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DVSTCTR0 to value 0"]
impl crate::Resettable for DVSTCTR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
