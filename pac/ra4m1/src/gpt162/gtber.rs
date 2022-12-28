#[doc = "Register `GTBER` reader"]
pub struct R(crate::R<GTBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTBER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTBER` writer"]
pub struct W(crate::W<GTBER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTBER_SPEC>;
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
impl From<crate::W<GTBER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTBER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BD` reader - BD\\[1\\]: GTPR Buffer Operation Disable BD\\[0\\]: GTCCR Buffer Operation Disable"]
pub type BD_R = crate::FieldReader<u8, BD_A>;
#[doc = "BD\\[1\\]: GTPR Buffer Operation Disable BD\\[0\\]: GTCCR Buffer Operation Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BD_A {
    #[doc = "0: Buffer operation is enabled"]
    _0 = 0,
    #[doc = "1: Buffer operation is disabled"]
    _1 = 1,
}
impl From<BD_A> for u8 {
    #[inline(always)]
    fn from(variant: BD_A) -> Self {
        variant as _
    }
}
impl BD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BD_A> {
        match self.bits {
            0 => Some(BD_A::_0),
            1 => Some(BD_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BD_A::_1
    }
}
#[doc = "Field `BD` writer - BD\\[1\\]: GTPR Buffer Operation Disable BD\\[0\\]: GTCCR Buffer Operation Disable"]
pub type BD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTBER_SPEC, u8, BD_A, 2, O>;
impl<'a, const O: u8> BD_W<'a, O> {
    #[doc = "Buffer operation is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BD_A::_0)
    }
    #[doc = "Buffer operation is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BD_A::_1)
    }
}
#[doc = "Field `CCRA` reader - GTCCRA Buffer Operation"]
pub type CCRA_R = crate::FieldReader<u8, CCRA_A>;
#[doc = "GTCCRA Buffer Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCRA_A {
    #[doc = "0: Buffer operation is not performed"]
    _00 = 0,
    #[doc = "1: Single buffer operation (GTCCRA <--> GTCCRC)"]
    _01 = 1,
    #[doc = "2: Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)"]
    _10 = 2,
    #[doc = "3: Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)"]
    _11 = 3,
}
impl From<CCRA_A> for u8 {
    #[inline(always)]
    fn from(variant: CCRA_A) -> Self {
        variant as _
    }
}
impl CCRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCRA_A {
        match self.bits {
            0 => CCRA_A::_00,
            1 => CCRA_A::_01,
            2 => CCRA_A::_10,
            3 => CCRA_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CCRA_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CCRA_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CCRA_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CCRA_A::_11
    }
}
#[doc = "Field `CCRA` writer - GTCCRA Buffer Operation"]
pub type CCRA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTBER_SPEC, u8, CCRA_A, 2, O>;
impl<'a, const O: u8> CCRA_W<'a, O> {
    #[doc = "Buffer operation is not performed"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CCRA_A::_00)
    }
    #[doc = "Single buffer operation (GTCCRA <--> GTCCRC)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CCRA_A::_01)
    }
    #[doc = "Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CCRA_A::_10)
    }
    #[doc = "Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CCRA_A::_11)
    }
}
#[doc = "Field `CCRB` reader - GTCCRB Buffer Operation"]
pub type CCRB_R = crate::FieldReader<u8, CCRB_A>;
#[doc = "GTCCRB Buffer Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCRB_A {
    #[doc = "0: Buffer operation is not performed"]
    _00 = 0,
    #[doc = "1: Single buffer operation (GTCCRB <--> GTCCRE)"]
    _01 = 1,
    #[doc = "2: Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)"]
    _10 = 2,
    #[doc = "3: Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)"]
    _11 = 3,
}
impl From<CCRB_A> for u8 {
    #[inline(always)]
    fn from(variant: CCRB_A) -> Self {
        variant as _
    }
}
impl CCRB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCRB_A {
        match self.bits {
            0 => CCRB_A::_00,
            1 => CCRB_A::_01,
            2 => CCRB_A::_10,
            3 => CCRB_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CCRB_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CCRB_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CCRB_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CCRB_A::_11
    }
}
#[doc = "Field `CCRB` writer - GTCCRB Buffer Operation"]
pub type CCRB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTBER_SPEC, u8, CCRB_A, 2, O>;
impl<'a, const O: u8> CCRB_W<'a, O> {
    #[doc = "Buffer operation is not performed"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CCRB_A::_00)
    }
    #[doc = "Single buffer operation (GTCCRB <--> GTCCRE)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CCRB_A::_01)
    }
    #[doc = "Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CCRB_A::_10)
    }
    #[doc = "Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CCRB_A::_11)
    }
}
#[doc = "Field `PR` reader - GTPR Buffer Operation"]
pub type PR_R = crate::FieldReader<u8, PR_A>;
#[doc = "GTPR Buffer Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PR_A {
    #[doc = "0: Buffer operation is not performed"]
    _00 = 0,
    #[doc = "1: Single buffer operation (GTPBR --> GTPR)"]
    _01 = 1,
}
impl From<PR_A> for u8 {
    #[inline(always)]
    fn from(variant: PR_A) -> Self {
        variant as _
    }
}
impl PR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PR_A> {
        match self.bits {
            0 => Some(PR_A::_00),
            1 => Some(PR_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PR_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PR_A::_01
    }
}
#[doc = "Field `PR` writer - GTPR Buffer Operation"]
pub type PR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTBER_SPEC, u8, PR_A, 2, O>;
impl<'a, const O: u8> PR_W<'a, O> {
    #[doc = "Buffer operation is not performed"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PR_A::_00)
    }
    #[doc = "Single buffer operation (GTPBR --> GTPR)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PR_A::_01)
    }
}
#[doc = "GTCCRA and GTCCRB Forcible Buffer Operation This bit is read as 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRSWT_AW {
    #[doc = "0: no effect"]
    _0 = 0,
    #[doc = "1: Forcibly performs buffer transfer of GTCCRA and GTCCRB. This bit automatically returns to 0 after the writing of 1."]
    _1 = 1,
}
impl From<CCRSWT_AW> for bool {
    #[inline(always)]
    fn from(variant: CCRSWT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCRSWT` writer - GTCCRA and GTCCRB Forcible Buffer Operation This bit is read as 0."]
pub type CCRSWT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER_SPEC, CCRSWT_AW, O>;
impl<'a, const O: u8> CCRSWT_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCRSWT_AW::_0)
    }
    #[doc = "Forcibly performs buffer transfer of GTCCRA and GTCCRB. This bit automatically returns to 0 after the writing of 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCRSWT_AW::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - BD\\[1\\]: GTPR Buffer Operation Disable BD\\[0\\]: GTCCR Buffer Operation Disable"]
    #[inline(always)]
    pub fn bd(&self) -> BD_R {
        BD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - GTCCRA Buffer Operation"]
    #[inline(always)]
    pub fn ccra(&self) -> CCRA_R {
        CCRA_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GTCCRB Buffer Operation"]
    #[inline(always)]
    pub fn ccrb(&self) -> CCRB_R {
        CCRB_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GTPR Buffer Operation"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - BD\\[1\\]: GTPR Buffer Operation Disable BD\\[0\\]: GTCCR Buffer Operation Disable"]
    #[inline(always)]
    #[must_use]
    pub fn bd(&mut self) -> BD_W<0> {
        BD_W::new(self)
    }
    #[doc = "Bits 16:17 - GTCCRA Buffer Operation"]
    #[inline(always)]
    #[must_use]
    pub fn ccra(&mut self) -> CCRA_W<16> {
        CCRA_W::new(self)
    }
    #[doc = "Bits 18:19 - GTCCRB Buffer Operation"]
    #[inline(always)]
    #[must_use]
    pub fn ccrb(&mut self) -> CCRB_W<18> {
        CCRB_W::new(self)
    }
    #[doc = "Bits 20:21 - GTPR Buffer Operation"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<20> {
        PR_W::new(self)
    }
    #[doc = "Bit 22 - GTCCRA and GTCCRB Forcible Buffer Operation This bit is read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn ccrswt(&mut self) -> CCRSWT_W<22> {
        CCRSWT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Buffer Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtber](index.html) module"]
pub struct GTBER_SPEC;
impl crate::RegisterSpec for GTBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtber::R](R) reader structure"]
impl crate::Readable for GTBER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtber::W](W) writer structure"]
impl crate::Writable for GTBER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTBER to value 0"]
impl crate::Resettable for GTBER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
